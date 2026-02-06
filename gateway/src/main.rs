use axum::{
    Router,
    routing::{post, put},
};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::{net::TcpListener, select, sync::broadcast, time::sleep};

mod api;
mod config;
mod device;
mod dispatcher;
mod lifecycle;
mod logging;
mod modbus;
mod mqtt;
mod state;

use crate::lifecycle::Lifecycle;
use crate::modbus::ModbusPoller;
use crate::mqtt::MqttPublisher;
use crate::state::{AppState, GatewayEvent, GatewayState};
use crate::{
    api::{create_device, delete_device, get_devices, update_device},
    state::StateListener,
};
use crate::{config::Config, dispatcher::Dispatcher};
use tracing::{debug, error, info, warn};

fn spawn_service<T: Lifecycle>(service: T, shutdown: broadcast::Receiver<()>) {
    tokio::spawn(async move {
        service.run(shutdown).await;
    });
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    let config = Config::load_or_default();
    info!(
        "Config loaded: server={}:{}, mqtt={}:{}",
        config.server.host, config.server.port, config.mqtt.broker, config.mqtt.port
    );
    info!("Polling: {}ms", config.polling.interval_ms);

    let (shutdown_tx, _shutdown_rx) = tokio::sync::broadcast::channel::<()>(1);

    // Event channel für Gateway-Events
    let (tx, mut rx) = tokio::sync::mpsc::channel::<GatewayEvent>(32);

    // Gemeinsamer State im RAM
    // Arc = mehrere Tasks dürfen ihn "besitzen"
    // Mutex = jeweils nur EIN Task darf ihn ändern
    let shared_state = Arc::new(Mutex::new(GatewayState::new()));

    let mut listeners: Vec<Arc<dyn StateListener>> = Vec::new();
    listeners.push(Arc::new(logging::ConsoleLogger::new()));

    let mqtt_service = match MqttPublisher::new(
        &config.mqtt.broker,
        config.mqtt.port,
        &config.mqtt.client_id,
    )
    .await
    {
        Ok(p) => {
            info!("✓ MQTT connected successfully");
            Some(Arc::new(p))
        }
        Err(err) => {
            warn!("✗ MQTT connection failed, running without MQTT: {}", err);
            None
        }
    };

    if let Some(mqtt) = &mqtt_service {
        listeners.push(mqtt.clone()); // mqtt is already an Arc
    }

    let dispatcher = Arc::new(Dispatcher::new(listeners));

    tx.send(GatewayEvent::DeviceCreated { id: 1 })
        .await
        .unwrap();
    tx.send(GatewayEvent::DeviceCreated { id: 2 })
        .await
        .unwrap();
    tx.send(GatewayEvent::DeviceValueObserved {
        id: 1,
        value: Some(rand::random::<f64>() * 100.0),
    })
    .await
    .unwrap();
    tx.send(GatewayEvent::DeviceValueObserved {
        id: 2,
        value: Some(rand::random::<f64>() * 100.0),
    })
    .await
    .unwrap();

    // -------------------------
    // EVENT-LOOP TASK
    // -------------------------
    let event_state = shared_state.clone();

    let event_dispatcher = dispatcher.clone(); // Arc klonen für den Task

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

    // -------------------------
    // MODBUS POLLING TASK
    // -------------------------
    if config.modbus.enabled {
        let modbus_poller = ModbusPoller::new(config.modbus.clone(), tx.clone());
        let shutdown_rx = shutdown_tx.subscribe();
        spawn_service(modbus_poller, shutdown_rx);
        info!("Modbus polling task started");
    }

    // -------------------------
    // BACKGROUND TICK TASK
    // -------------------------
    // task creates only events, not changing the state
    if config.simulation.enabled {
        let tx2 = tx.clone();
        let mut tick_shutdown = shutdown_tx.subscribe();
        let event_state = shared_state.clone();
        let snapshot = {
            match event_state.lock() {
                Ok(state) => state.devices.clone(),
                Err(_) => {
                    error!("Mutex poisoned in simulation task");
                    return; // Task beenden
                }
            }
        };
        tokio::spawn(async move {
            loop {
                select! {
                    _ = sleep(Duration::from_millis(config.simulation.interval_ms)) => {
                        for dev in &snapshot {
                            if let Some(val) = dev.value {
                                 let new_value = val + config.simulation.add_value as f64;
                            let _ = tx2.send(GatewayEvent::DeviceValueObserved {
                                id: dev.id,
                                value: Some(new_value),
                            })
                            .await;
                            }

                        }
                    }

                    _ = tick_shutdown.recv() => {
                        info!("Background tick task shutting down");
                        break;
                    }
                }
            }

            info!("Background tick task stopped");
        });
    }

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

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = TcpListener::bind(&addr).await.unwrap();

    info!("HTTP server listening on {}", addr);

    let shutdown_signal = shutdown_tx.clone();

    tokio::spawn(async move {
        // wait for Ctrl+C
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to listen for Ctrl+C");

        info!("Shutdown signal received (Ctrl+C)");

        // send shutdown signal to all listener
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
