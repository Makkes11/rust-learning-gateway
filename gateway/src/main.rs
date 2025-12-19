use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{Router, routing::post};
use device::Device;
use std::sync::{Arc, Mutex};
use std::time::Duration;

mod device;

use device::GatewayState;

use crate::device::{AppState, DeviceInput, GatewayEvent};

#[tokio::main]
async fn main() {
    // mpsc::channel erzeugt:
    // tx = Sender -> erzeugt Events
    // rx = Receiver -> verarbeitet Events
    let (tx, mut rx) = tokio::sync::mpsc::channel::<GatewayEvent>(32);

    // Gemeinsamer State im RAM
    // Arc = mehrere Tasks dürfen ihn "besitzen"
    // Mutex = jeweils nur EIN Task darf ihn ändern
    let shared_state = Arc::new(Mutex::new(GatewayState::new()));

    // Optional: Startzustand setzen (direkt, ohne Events)
    {
        let mut guard = shared_state.lock().unwrap();
        guard.apply_event(GatewayEvent::Update {
            id: (1),
            value: (rand::random::<i32>() % 100),
        });
        guard.apply_event(GatewayEvent::Update {
            id: (2),
            value: (rand::random::<i32>() % 100),
        });
    }

    // -------------------------
    // EVENT-LOOP TASK
    // -------------------------
    // Dieser Task ist der EINZIGE Ort, an dem der State verändert wird.
    let event_state = shared_state.clone();

    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
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
        for _ in 0..10 {
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
        .with_state(app_state);

    println!("Server running on http://127.0.0.1:3000");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn get_devices(State(app): State<AppState>) -> Result<Json<Vec<Device>>, StatusCode> {
    let state = app
        .state
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(state.devices.clone()))
}

async fn create_or_update_device(
    State(app): State<AppState>,
    Json(payload): Json<DeviceInput>,
) -> Result<Json<Device>, StatusCode> {
    app.tx
        .send(GatewayEvent::Update {
            id: payload.id,
            value: payload.value,
        })
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(Device {
        id: payload.id,
        value: payload.value,
    }))
}
