#[cfg(test)]
mod tests {
    use chrono::Utc;
    use gateway::core::state::GatewayState;
    use gateway::core::{
        dispatcher::Dispatcher,
        events::GatewayEvent,
        state::{ListenerError, StateChange, StateListener},
    };
    use std::sync::Arc;
    use tokio::sync::Mutex;

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

    #[tokio::test]
    async fn dispatcher_notifies_all_listeners() {
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

        let change = state
            .apply_event(event)
            .expect("state error")
            .expect("no change");

        dispatcher.dispatch(change).await;

        let events = recorded.lock().await;

        assert_eq!(events.len(), 1);
    }
}
