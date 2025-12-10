use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{Router, routing::post};
use device::Device;
use std::sync::{Arc, Mutex};
use std::time::Duration;

mod device;

use device::GatewayState;

use crate::device::{DeviceInput, GatewayEvent};

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(GatewayState::new()));
    let bg_state = Arc::clone(&state);
    if let Ok(mut guard) = bg_state.lock() {
        guard.apply_event(GatewayEvent::Update {
            id: (1),
            value: (rand::random::<i32>() % 100),
        });
        guard.apply_event(GatewayEvent::Update {
            id: (2),
            value: (rand::random::<i32>() % 100),
        });
    }
    tokio::spawn(async move {
        for _ in 0..10 {
            tokio::time::sleep(Duration::from_millis(500)).await;
            if let Ok(mut guard) = bg_state.lock() {
                guard.apply_event(GatewayEvent::Tick(1));
                println!("{:?}", guard.devices);
            }
        }
    });

    // router
    let app = Router::new()
        .route("/devices", post(create_or_update_device).get(get_devices))
        .with_state(state);

    println!("Server running on http://127.0.0.1:3000");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn get_devices(
    State(state): State<Arc<Mutex<GatewayState>>>,
) -> Result<Json<Vec<Device>>, StatusCode> {
    let state = state
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(state.devices.clone()))
}

async fn create_or_update_device(
    State(state): State<Arc<Mutex<GatewayState>>>,
    Json(payload): Json<DeviceInput>,
) -> Result<Json<Device>, StatusCode> {
    let mut state = state
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    state.apply_event(GatewayEvent::Update {
        id: payload.id,
        value: payload.value,
    });

    Ok(Json(Device {
        id: payload.id,
        value: payload.value,
    }))
}
