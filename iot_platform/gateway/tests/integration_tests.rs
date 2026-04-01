use chrono::Utc;
#[cfg(test)]
use gateway::core::state::GatewayState;
use gateway::core::{
    dispatcher::Dispatcher,
    events::GatewayEvent,
    state::{ListenerError, StateChange, StateListener},
};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::test]
async fn event_flows_from_state_to_listener() {
    let recorded = Arc::new(Mutex::new(Vec::new()));
    let listener = Arc::new(MockListener {
        events: recorded.clone(),
    });
    let dispatcher = Dispatcher::new(vec![listener]);

    let mut state = GatewayState::new();
    let ts = Utc::now().timestamp_millis();
    let event = GatewayEvent::DeviceCreated {
        id: 1,
        timestamp: ts,
    };

    let change = state.apply_event(event).expect("state error");

    dispatcher.dispatch(change).await;

    let events = recorded.lock().await;
    assert_eq!(events.len(), 1);
}

#[tokio::test]
async fn event_flows_through_event_loop() {
    let recorded = Arc::new(Mutex::new(Vec::new()));
    let listener = Arc::new(MockListener {
        events: recorded.clone(),
    });
    let dispatcher = Dispatcher::new(vec![listener]);

    let state = Arc::new(Mutex::new(GatewayState::new()));

    let event = GatewayEvent::DeviceCreated {
        id: 42,
        timestamp: Utc::now().timestamp_millis(),
    };

    let mut s = state.lock().await;
    let change = s.apply_event(event).expect("state error");
    drop(s);

    dispatcher.dispatch(change).await;

    let events = recorded.lock().await;
    assert_eq!(events.len(), 1);
}

#[tokio::test]
async fn api_create_device_triggers_state_and_dispatcher() {
    let state = Arc::new(Mutex::new(GatewayState::new()));
    let listener = Arc::new(TestListener {
        state: state.clone(),
    });
    let dispatcher = Dispatcher::new(vec![listener]);

    // Device Created Event
    let device_id = 42;
    let ts_created = Utc::now().timestamp_millis();
    {
        let mut s = state.lock().await;
        let change = s
            .apply_event(GatewayEvent::DeviceCreated {
                id: device_id,
                timestamp: ts_created,
            })
            .expect("state error");
        drop(s);
        dispatcher.dispatch(change).await;
    }

    // Device Value Update Event
    let new_value = 123.45;
    let ts_updated = Utc::now().timestamp_millis();
    {
        let mut s = state.lock().await;
        let change = s
            .apply_event(GatewayEvent::DeviceValueObserved {
                id: device_id,
                value: new_value,
                timestamp: ts_updated,
            })
            .expect("state error");
        drop(s);
        dispatcher.dispatch(change).await;
    }

    let s = state.lock().await;
    assert_eq!(s.devices.len(), 1);
    let dev = &s.devices[0];
    assert_eq!(dev.id, device_id);
    assert_eq!(dev.value, Some(new_value));
    assert_eq!(dev.timestamp, ts_updated);
}

struct MockListener {
    events: Arc<Mutex<Vec<StateChange>>>,
}

#[async_trait::async_trait]
impl StateListener for MockListener {
    async fn on_event(&self, event: StateChange) -> Result<(), ListenerError> {
        self.events.lock().await.push(event);
        Ok(())
    }
}

struct TestListener {
    state: Arc<Mutex<GatewayState>>,
}

#[async_trait::async_trait]
impl StateListener for TestListener {
    async fn on_event(&self, event: StateChange) -> Result<(), ListenerError> {
        let mut s = self.state.lock().await;
        match event {
            StateChange::DeviceCreated { id, timestamp } => {
                s.apply_event(GatewayEvent::DeviceCreated { id, timestamp })
                    .unwrap();
            }
            StateChange::DeviceUpdated {
                id,
                value,
                timestamp,
            } => {
                s.apply_event(GatewayEvent::DeviceValueObserved {
                    id,
                    value,
                    timestamp,
                })
                .unwrap();
            }
            StateChange::DeviceRemoved { id, timestamp } => {
                s.apply_event(GatewayEvent::DeviceRemoved { id, timestamp })
                    .unwrap();
            }
        }
        Ok(())
    }
}
