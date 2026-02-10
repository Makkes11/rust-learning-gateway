use axum::{
    Router,
    routing::{post, put},
};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

mod adapters;
mod config;
mod core;
mod logging;

use crate::core::state::{AppState, GatewayEvent, GatewayState};
use crate::{
    adapters::api::{create_device, delete_device, get_devices, update_device},
    core::state::StateListener,
};
use crate::{adapters::mqtt::MqttPublisher, config::SourceMode};
use crate::{
    adapters::{modbus::ModbusPoller, simulation::SimulationPoller, spawn_service::spawn_service},
    core::bootstrap::initialize_devices,
};
use crate::{config::Config, core::dispatcher::Dispatcher};
use tracing::{debug, error, info, warn};

#[tokio::main]
async fn main() {
    // -------------------------
    // TRACING / LOGGING
    // -------------------------
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    // -------------------------
    // LOAD CONFIG
    // -------------------------
    let config = Config::load_or_default();
    info!(
        "Config loaded: server={}:{}, mqtt={}:{}, mode={:?}",
        config.api.host, config.api.port, config.mqtt.broker, config.mqtt.port, config.mode
    );
    info!("Polling: {}ms", config.modbus.poll_interval_ms);

    // -------------------------
    // SHUTDOWN CHANNEL
    // -------------------------
    let (shutdown_tx, _shutdown_rx) = tokio::sync::broadcast::channel::<()>(1);

    // -------------------------
    // EVENT CHANNEL & SHARED STATE
    // -------------------------
    let (tx, mut rx) = tokio::sync::mpsc::channel::<GatewayEvent>(32);
    let shared_state = Arc::new(Mutex::new(GatewayState::new()));

    // -------------------------
    // LISTENERS
    // -------------------------
    let mut listeners: Vec<Arc<dyn StateListener>> = vec![Arc::new(logging::ConsoleLogger::new())];

    let mqtt_service = match MqttPublisher::new(
        &config.mqtt.broker,
        config.mqtt.port,
        &config.mqtt.client_id,
    )
    .await
    {
        Ok(p) => Some(Arc::new(p)),
        Err(err) => {
            warn!("MQTT connection failed, running without MQTT: {}", err);
            None
        }
    };

    if let Some(mqtt) = &mqtt_service {
        listeners.push(mqtt.clone());
    }

    let dispatcher = Arc::new(Dispatcher::new(listeners));

    // -------------------------
    // INITIALIZE DEVICES
    // -------------------------
    initialize_devices(&tx, &config).await;

    // -------------------------
    // POLLER TASK (MODBUS / SIMULATION)
    // -------------------------
    info!("Starting gateway with mode: {:?}", config.mode);
    match config.mode {
        SourceMode::Modbus => {
            let modbus = ModbusPoller::new(config.modbus.clone(), tx.clone());
            spawn_service(modbus, shutdown_tx.subscribe());
        }
        SourceMode::Simulation => {
            let sim = SimulationPoller::new(config.simulation.clone(), tx.clone());
            spawn_service(sim, shutdown_tx.subscribe());
        }
    }

    // -------------------------
    // EVENT LOOP
    // -------------------------
    let event_state = shared_state.clone();
    let event_dispatcher = dispatcher.clone();

    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            debug!("Event loop: received {:?}", event);

            let state_change = {
                let mut state = match event_state.lock() {
                    Ok(s) => s,
                    Err(_) => {
                        error!("Mutex poisoned in event loop");
                        return;
                    }
                };

                match state.apply_event(event) {
                    Ok(sc) => sc,
                    Err(e) => {
                        error!("{}", e);
                        continue;
                    }
                }
            };

            if let Some(change) = state_change {
                event_dispatcher.dispatch(change).await;
            }
        }
    });

    let app_state = AppState {
        tx: tx.clone(),
        state: shared_state.clone(),
    };

    // -------------------------
    // ROUTER
    // -------------------------
    let app = Router::new()
        .route("/devices", post(update_device).get(get_devices))
        .route("/devices/{id}", put(create_device).delete(delete_device))
        .with_state(app_state);

    let addr = format!("{}:{}", config.api.host, config.api.port);
    let listener = TcpListener::bind(&addr).await.unwrap();
    info!("HTTP server listening on {}", addr);

    // -------------------------
    // GRACEFUL SHUTDOWN
    // -------------------------
    let shutdown_signal = shutdown_tx.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to listen for Ctrl+C");
        info!("Shutdown signal received (Ctrl+C)");
        let _ = shutdown_signal.send(());
    });
    info!("Press Ctrl+C to shutdown gracefully");

    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            let mut shutdown_rx = shutdown_tx.subscribe();
            shutdown_rx.recv().await.ok();
            info!("HTTP server shutting down");
        })
        .await
        .unwrap();

    info!("Gateway stopped completely");
}
