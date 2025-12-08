use axum::Json;
use axum::extract::State;
use axum::{Router, routing::get};
use device::Device;
use std::sync::{Arc, Mutex};

mod device;

use device::GatewayState;

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(GatewayState::new()));

    // router
    let app = Router::new()
        .route("/devices", get(get_devices))
        .with_state(state);

    println!("Server running on http://127.0.0.1:3000");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn get_devices(State(state): State<Arc<Mutex<GatewayState>>>) -> Json<Vec<Device>> {
    let state = state.lock().unwrap();
    Json(state.devices.clone())
}
