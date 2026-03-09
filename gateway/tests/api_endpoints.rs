use axum::body;
use axum::{body::Body, http::Request, http::StatusCode, Router};
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

// Helper to create a fresh router for each test
fn create_test_router(
    state: Arc<Mutex<GatewayState>>,
    tx: tokio::sync::mpsc::Sender<gateway::core::events::GatewayEvent>,
) -> Router {
    let app_state = AppState { tx, state };
    Router::new()
        .route(
            "/devices",
            axum::routing::post(create_device).get(get_devices),
        )
        .route(
            "/devices/{id}",
            axum::routing::put(update_device).delete(delete_device),
        )
        .with_state(app_state)
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

#[tokio::test]
async fn error_update_nonexistent_device() {
    // Setup
    let state = Arc::new(Mutex::new(GatewayState::new()));
    let (tx, mut rx) = tokio::sync::mpsc::channel::<gateway::core::events::GatewayEvent>(10);
    let router = create_test_router(state.clone(), tx);

    // Try to update a device that doesn't exist
    let update_json = r#"{"id":999,"value":42.5}"#;
    let request = Request::builder()
        .method("PUT")
        .uri("/devices/999")
        .header("content-type", "application/json")
        .body(Body::from(update_json))
        .unwrap();

    let response = router.clone().oneshot(request).await.unwrap();
    // API returns 200 but the state won't update since device doesn't exist
    assert_eq!(response.status(), StatusCode::OK);

    process_events(&mut rx, &state).await;

    // Verify no devices were created (state won't apply update for non-existent device)
    {
        let state_guard = state.lock().await;
        assert_eq!(state_guard.devices.len(), 0);
    }
}

#[tokio::test]
async fn error_delete_nonexistent_device() {
    // Setup
    let state = Arc::new(Mutex::new(GatewayState::new()));
    let (tx, mut rx) = tokio::sync::mpsc::channel::<gateway::core::events::GatewayEvent>(10);
    let router = create_test_router(state.clone(), tx);

    // Try to delete a device that doesn't exist
    let request = Request::builder()
        .method("DELETE")
        .uri("/devices/999")
        .body(Body::empty())
        .unwrap();

    let response = router.clone().oneshot(request).await.unwrap();
    // API returns 204 NO_CONTENT (event sent successfully)
    // State won't apply deletion since device doesn't exist
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    process_events(&mut rx, &state).await;

    // Verify state is still empty
    {
        let state_guard = state.lock().await;
        assert_eq!(state_guard.devices.len(), 0);
    }
}

#[tokio::test]
async fn error_invalid_json_in_create() {
    // Setup
    let state = Arc::new(Mutex::new(GatewayState::new()));
    let (tx, _rx) = tokio::sync::mpsc::channel::<gateway::core::events::GatewayEvent>(10);
    let router = create_test_router(state.clone(), tx);

    // Send invalid JSON
    let request = Request::builder()
        .method("POST")
        .uri("/devices")
        .header("content-type", "application/json")
        .body(Body::from(r#"{ invalid json }"#))
        .unwrap();

    let response = router.clone().oneshot(request).await.unwrap();
    // Axum returns 400 BAD_REQUEST for malformed JSON
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    // Verify no device was created
    {
        let state_guard = state.lock().await;
        assert_eq!(state_guard.devices.len(), 0);
    }
}

#[tokio::test]
async fn error_missing_field_in_create() {
    // Setup
    let state = Arc::new(Mutex::new(GatewayState::new()));
    let (tx, _rx) = tokio::sync::mpsc::channel::<gateway::core::events::GatewayEvent>(10);
    let router = create_test_router(state.clone(), tx);

    // Missing 'value' field
    let request = Request::builder()
        .method("POST")
        .uri("/devices")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"id":1}"#))
        .unwrap();

    let response = router.clone().oneshot(request).await.unwrap();
    // Should fail due to missing required field
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    // Verify no device was created
    {
        let state_guard = state.lock().await;
        assert_eq!(state_guard.devices.len(), 0);
    }
}

#[tokio::test]
async fn error_invalid_type_in_create() {
    // Setup
    let state = Arc::new(Mutex::new(GatewayState::new()));
    let (tx, _rx) = tokio::sync::mpsc::channel::<gateway::core::events::GatewayEvent>(10);
    let router = create_test_router(state.clone(), tx);

    // 'value' should be a number, not a string
    let request = Request::builder()
        .method("POST")
        .uri("/devices")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"id":1,"value":"not_a_number"}"#))
        .unwrap();

    let response = router.clone().oneshot(request).await.unwrap();
    // Should fail due to type mismatch
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    // Verify no device was created
    {
        let state_guard = state.lock().await;
        assert_eq!(state_guard.devices.len(), 0);
    }
}

#[tokio::test]
async fn error_empty_body_in_create() {
    // Setup
    let state = Arc::new(Mutex::new(GatewayState::new()));
    let (tx, _rx) = tokio::sync::mpsc::channel::<gateway::core::events::GatewayEvent>(10);
    let router = create_test_router(state.clone(), tx);

    // Empty body
    let request = Request::builder()
        .method("POST")
        .uri("/devices")
        .header("content-type", "application/json")
        .body(Body::empty())
        .unwrap();

    let response = router.clone().oneshot(request).await.unwrap();
    // Axum returns 400 BAD_REQUEST for empty body when expecting JSON
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    // Verify no device was created
    {
        let state_guard = state.lock().await;
        assert_eq!(state_guard.devices.len(), 0);
    }
}

#[tokio::test]
async fn error_create_duplicate_device_updates_value() {
    // Setup
    let state = Arc::new(Mutex::new(GatewayState::new()));
    let (tx, mut rx) = tokio::sync::mpsc::channel::<gateway::core::events::GatewayEvent>(10);
    let router = create_test_router(state.clone(), tx);

    // Create device first time
    let device_json = r#"{"id":1,"value":10.0}"#;
    let request = Request::builder()
        .method("POST")
        .uri("/devices")
        .header("content-type", "application/json")
        .body(Body::from(device_json))
        .unwrap();

    let response = router.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    process_events(&mut rx, &state).await;

    // Try to create the same device again with different value
    let device_json2 = r#"{"id":1,"value":20.0}"#;
    let request2 = Request::builder()
        .method("POST")
        .uri("/devices")
        .header("content-type", "application/json")
        .body(Body::from(device_json2))
        .unwrap();

    let response2 = router.clone().oneshot(request2).await.unwrap();
    // Should succeed - API sends the events
    assert_eq!(response2.status(), StatusCode::OK);
    process_events(&mut rx, &state).await;

    // Verify device exists with updated value
    {
        let state_guard = state.lock().await;
        assert_eq!(state_guard.devices.len(), 1);
        assert_eq!(state_guard.devices[0].id, 1);
        // Second create sends DeviceValueObserved event which updates the value
        assert_eq!(state_guard.devices[0].value, Some(20.0));
    }
}
