use crate::core::device::{Device, DeviceInput};
use crate::core::events::GatewayEvent;
use crate::core::state::AppState;
use axum::extract::Path;
use axum::{Json, extract::State, http::StatusCode};
use tracing::info;

pub async fn get_devices(State(app): State<AppState>) -> Json<Vec<Device>> {
    let state = app.state.lock().await;
    Json(state.devices.clone())
}

pub async fn create_device(
    State(app): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<Device>, StatusCode> {
    info!("API: Creating device id={}", id);
    let timestamp = chrono::Utc::now();

    app.tx
        .send(GatewayEvent::DeviceCreated {
            id: id,
            timestamp: timestamp,
        })
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(Device {
        id: id,
        value: None,
        timestamp: timestamp,
    }))
}

pub async fn update_device(
    State(app): State<AppState>,
    Json(payload): Json<DeviceInput>,
) -> Result<Json<Device>, StatusCode> {
    info!("API: Updating device id={}", payload.id);

    let timestamp = chrono::Utc::now();

    app.tx
        .send(GatewayEvent::DeviceValueObserved {
            id: payload.id,
            value: Some(payload.value),
            timestamp: timestamp,
        })
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(Device {
        id: payload.id,
        value: Some(payload.value),
        timestamp: timestamp,
    }))
}

pub async fn delete_device(
    State(app): State<AppState>,
    Path(id): Path<u32>,
) -> Result<StatusCode, StatusCode> {
    info!("API: Deleting device id={}", id);

    app.tx
        .send(GatewayEvent::DeviceRemoved {
            id: id,
            timestamp: chrono::Utc::now(),
        })
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}
