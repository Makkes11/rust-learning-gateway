use crate::device::{Device, DeviceInput};
use crate::state::{AppState, GatewayEvent};
use axum::{Json, extract::State, http::StatusCode};

pub async fn get_devices(State(app): State<AppState>) -> Result<Json<Vec<Device>>, StatusCode> {
    let state = app
        .state
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(state.devices.clone()))
}

pub async fn create_or_update_device(
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
