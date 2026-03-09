use axum::body;
use axum::{Router, body::Body, http::Request, http::StatusCode};
use gateway::adapters::api::{create_device, delete_device, get_devices, update_device};
use gateway::core::state::{AppState, GatewayState};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower::ServiceExt;

// Helper to process pending events from the channel
async fn process_events(
    rx: &mut tokio::sync::mpsc::Receiver<gateway::core::events::GatewayEvent>,
    state: &Arc<Mutex<GatewayState>>,
) {
    while let Ok(event) = rx.try_recv() {
        let mut state_guard = state.lock().await;
        let _ = state_guard.apply_event(event);
    }
}

#[tokio::test]
async fn api_endpoints_work() {
    // Setup: Create shared state
    let state = Arc::new(Mutex::new(GatewayState::new()));

    // Create a channel to receive events
    let (tx, mut rx) = tokio::sync::mpsc::channel::<gateway::core::events::GatewayEvent>(10);

    let app_state = AppState {
        tx,
        state: state.clone(),
    };

    // Build router
    let router = Router::new()
        .route(
            "/devices",
            axum::routing::post(create_device).get(get_devices),
        )
        .route(
            "/devices/{id}",
            axum::routing::put(update_device).delete(delete_device),
        )
        .with_state(app_state);

    // TEST 1: Create device via POST /devices
    let device_json = r#"{"id":1,"value":12.34}"#;
    let request = Request::builder()
        .method("POST")
        .uri("/devices")
        .header("content-type", "application/json")
        .body(Body::from(device_json))
        .unwrap();

    let response = router.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Process pending events from the channel
    process_events(&mut rx, &state).await;

    // Verify device was created
    {
        let state_guard = state.lock().await;
        assert_eq!(state_guard.devices.len(), 1);
        assert_eq!(state_guard.devices[0].id, 1);
        assert_eq!(state_guard.devices[0].value, Some(12.34));
    }

    // TEST 2: Get devices via GET /devices
    let request = Request::builder()
        .method("GET")
        .uri("/devices")
        .body(Body::empty())
        .unwrap();

    let response = router.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = body::to_bytes(response.into_body(), 1024 * 1024)
        .await
        .unwrap();
    let devices: Vec<Value> = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(devices.len(), 1);
    assert_eq!(devices[0]["id"], 1);
    assert_eq!(devices[0]["value"], 12.34);

    // TEST 3: Update device via PUT /devices/{id}
    let update_json = r#"{"id":1,"value":56.78}"#;
    let request = Request::builder()
        .method("PUT")
        .uri("/devices/1")
        .header("content-type", "application/json")
        .body(Body::from(update_json))
        .unwrap();

    let response = router.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Process pending events from the channel
    process_events(&mut rx, &state).await;

    // Verify device was updated
    {
        let state_guard = state.lock().await;
        assert_eq!(state_guard.devices.len(), 1);
        assert_eq!(state_guard.devices[0].id, 1);
        assert_eq!(state_guard.devices[0].value, Some(56.78));
    }

    // TEST 4: Delete device via DELETE /devices/{id}
    let request = Request::builder()
        .method("DELETE")
        .uri("/devices/1")
        .body(Body::empty())
        .unwrap();

    let response = router.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // Process pending events from the channel
    process_events(&mut rx, &state).await;

    // Verify device was deleted
    {
        let state_guard = state.lock().await;
        assert_eq!(state_guard.devices.len(), 0);
    }
}
