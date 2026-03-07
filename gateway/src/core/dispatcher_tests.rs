#[cfg(test)]
mod tests {
    use crate::core::dispatcher::Dispatcher;
    use crate::core::events::GatewayEvent;
    use crate::core::state::{GatewayState, ListenerError, StateChange, StateListener};
    use chrono::Utc;
    use std::sync::{Arc, Mutex};

    struct MockListener {
        events: Arc<Mutex<Vec<StateChange>>>,
    }

    #[async_trait::async_trait]
    impl StateListener for MockListener {
        async fn on_event(&self, event: StateChange) -> Result<(), ListenerError> {
            self.events.lock().unwrap().push(event);
            Ok(())
        }
    }

    #[tokio::test]
    async fn dispatcher_notifies_all_listeners() {
        let recorded = Arc::new(Mutex::new(Vec::new()));

        let listener = Arc::new(MockListener {
            events: recorded.clone(),
        });

        let dispatcher = Dispatcher::new(vec![listener]);

        let mut state = GatewayState::new();

        let ts = Utc::now();

        let event = GatewayEvent::DeviceCreated {
            id: 1,
            timestamp: ts,
        };

        let change = state
            .apply_event(event)
            .expect("state error")
            .expect("no change");

        dispatcher.dispatch(change).await;

        let events = recorded.lock().unwrap();

        assert_eq!(events.len(), 1);
    }
}
