use axum::{
    Router,
    routing::{delete, post},
};
use std::sync::{Arc, Mutex};
use std::time::Duration;

mod api;
mod device;
mod mqtt;
mod state;

use crate::api::{create_or_update_device, delete_device, get_devices};
use crate::mqtt::MqttPublisher;
use crate::state::{AppState, GatewayEvent, GatewayState};

#[tokio::main]
async fn main() {
    // Event channel für Gateway-Events
    let (tx, mut rx) = tokio::sync::mpsc::channel::<GatewayEvent>(32);

    // Gemeinsamer State im RAM
    // Arc = mehrere Tasks dürfen ihn "besitzen"
    // Mutex = jeweils nur EIN Task darf ihn ändern
    let shared_state = Arc::new(Mutex::new(GatewayState::new()));

    let mqtt = match MqttPublisher::new("localhost", 1883, "rust-gateway").await {
        Ok(p) => {
            println!("✓ MQTT connected");
            Some(Arc::new(p))
        }
        Err(err) => {
            eprintln!("✗ MQTT failed: {}", err);
            None
        }
    };

    // Sende Startzustände devices mit Events --> auch in MQTT
    tx.send(GatewayEvent::Update {
        id: 1,
        value: rand::random::<i32>() % 100,
    })
    .await
    .unwrap();
    tx.send(GatewayEvent::Update {
        id: 2,
        value: rand::random::<i32>() % 100,
    })
    .await
    .unwrap();

    // -------------------------
    // EVENT-LOOP TASK
    // -------------------------
    // Dieser Task ist der EINZIGE Ort, an dem der State verändert wird.
    let event_state = shared_state.clone();

    let mqtt_clone = mqtt.clone();

    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            match &event {
                GatewayEvent::Update { id, value } => {
                    if let Some(mqtt) = &mqtt_clone {
                        mqtt.publish_device_update(*id, *value).await;
                    }
                }
                GatewayEvent::Remove(id) => {
                    if let Some(mqtt) = &mqtt_clone {
                        mqtt.delete_device(*id).await;
                    }
                }
                _ => {}
            }

            if let Ok(mut state) = event_state.lock() {
                state.apply_event(event);
            } else {
                eprintln!("Mutex poisoned");
            }
        }
    });

    // -------------------------
    // BACKGROUND TICK TASK
    // -------------------------
    // Dieser Task erzeugt nur Events, er verändert nicht den State.
    let tx2 = tx.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_millis(500)).await;

            tx2.send(GatewayEvent::Tick(1))
                .await
                .expect("worker disappeared");
        }
    });

    // AppState bündelt tx und shared_state
    let app_state = AppState {
        tx: tx.clone(),
        state: shared_state.clone(),
    };

    // -------------------------
    // ROUTER
    // -------------------------
    let app = Router::new()
        .route("/devices", post(create_or_update_device).get(get_devices))
        .route("/devices/{id}", delete(delete_device))
        .with_state(app_state);

    println!("Server running on http://127.0.0.1:3000");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
