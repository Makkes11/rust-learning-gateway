use crate::core::device::{Device, DeviceInput};
use crate::core::state::{AppState, GatewayEvent};
use axum::extract::Path;
use axum::{Json, extract::State, http::StatusCode};
use tracing::info;

pub async fn get_devices(State(app): State<AppState>) -> Result<Json<Vec<Device>>, StatusCode> {
    let state = app
        .state
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(state.devices.clone()))
}

pub async fn create_device(
    State(app): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<Device>, StatusCode> {
    info!("API: Creating device id={}", id);

    app.tx
        .send(GatewayEvent::DeviceCreated { id: id })
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(Device {
        id: id,
        value: None,
    }))
}

pub async fn update_device(
    State(app): State<AppState>,
    Json(payload): Json<DeviceInput>,
) -> Result<Json<Device>, StatusCode> {
    info!("API: Updating device id={}", payload.id);

    app.tx
        .send(GatewayEvent::DeviceValueObserved {
            id: payload.id,
            value: Some(payload.value),
        })
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(Device {
        id: payload.id,
        value: Some(payload.value),
    }))
}

pub async fn delete_device(
    State(app): State<AppState>,
    Path(id): Path<u32>,
) -> Result<StatusCode, StatusCode> {
    info!("API: Deleting device id={}", id);

    app.tx
        .send(GatewayEvent::Remove(id))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}
