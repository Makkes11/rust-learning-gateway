use axum::{
    routing::{post, put},
    Router,
};
use std::sync::Arc;
use tokio::{net::TcpListener, sync::Mutex};

mod adapters;
mod config;
mod core;
mod logging;

use crate::core::{
    events::GatewayEvent,
    state::{AppState, GatewayState, StateListener},
};
use crate::{
    adapters::api::{create_device, delete_device, get_devices, health_check, update_device},
    adapters::{
        modbus::ModbusPoller, mqtt::MqttPublisher, simulation::SimulationPoller,
        spawn_service::spawn_service,
    },
    config::{Config, SourceMode},
    core::bootstrap::initialize_devices,
    core::dispatcher::Dispatcher,
};
use tracing::{debug, error, info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // -------------------------
    // INITIALIZE TRACING / LOGGING
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
    info!("Full config:\n{}", toml::to_string_pretty(&config).unwrap());
    let gateway_name = config.gateway_name.clone();

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
    // LISTENERS SETUP
    // -------------------------
    let mut listeners: Vec<Arc<dyn StateListener>> =
        vec![Arc::new(logging::ConsoleLogger::new(&gateway_name))];

    let mqtt_service = match MqttPublisher::new(
        config.mqtt.clone(),
        config.gateway_id.clone(),
        config.gateway_name.clone(),
    )
    .await
    {
        Ok(p) => Some(Arc::new(p)),
        Err(err) => {
            warn!(
                "{}: {} - MQTT connection failed, running without MQTT: {}",
                gateway_name, config.mqtt.device_name, err
            );
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
    // START DATA SOURCES (MODBUS / SIMULATION)
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
    let gateway_name_loop = gateway_name.clone();

    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            debug!("{}: Event loop: received {:?}", gateway_name_loop, event);

            let state_change = {
                let mut state = event_state.lock().await;

                match state.apply_event(event) {
                    Ok(sc) => sc,
                    Err(e) => {
                        error!("{}: {}", gateway_name_loop, e);
                        continue;
                    }
                }
            };

            if let Some(change) = state_change {
                event_dispatcher.dispatch(change).await;
            }
        }
    });

    // -------------------------
    // APP STATE FOR ROUTES
    // -------------------------
    let app_state = AppState {
        tx: tx.clone(),
        state: shared_state.clone(),
    };
    let app = Router::new()
        .route("/devices", post(create_device).get(get_devices))
        .route("/devices/{id}", put(update_device).delete(delete_device))
        .route("/health", axum::routing::get(health_check))
        .with_state(app_state);

    let addr = format!("{}:{}", config.api.host, config.api.port);
    let listener = TcpListener::bind(&addr).await.unwrap();
    info!(
        "{}: {} - HTTP server listening on {}",
        gateway_name, config.api.device_name, addr
    );

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
    Ok(())
}
