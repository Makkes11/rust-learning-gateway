use crate::core::state::{StateChange, StateListener};
use std::sync::Arc;

pub struct Dispatcher {
    listeners: Vec<Arc<dyn StateListener>>,
}

impl Dispatcher {
    pub fn new(listeners: Vec<Arc<dyn StateListener>>) -> Self {
        Self { listeners }
    }

    pub async fn dispatch(&self, event: StateChange) {
        for listener in &self.listeners {
            // await on_event - sequential execution preserves listener order
            if let Err(e) = listener.on_event(event.clone()).await {
                // Centralized logging for all side-effect errors
                tracing::error!("Listener failed to process event: {:?}", e);
            };
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::core::state::{ListenerError, StateChange, StateListener};

    use std::sync::{Arc, Mutex};

    struct OkListener {
        calls: Arc<Mutex<u32>>,
    }

    #[async_trait::async_trait]
    impl StateListener for OkListener {
        async fn on_event(&self, _: StateChange) -> Result<(), ListenerError> {
            *self.calls.lock().unwrap() += 1;
            Ok(())
        }
    }

    struct FailingListener;

    #[async_trait::async_trait]
    impl StateListener for FailingListener {
        async fn on_event(&self, _: StateChange) -> Result<(), ListenerError> {
            Err(ListenerError::General("fail".into()))
        }
    }

    #[tokio::test]
    async fn dispatcher_continues_after_listener_failure() {
        let calls = Arc::new(Mutex::new(0));

        let dispatcher = Dispatcher::new(vec![
            Arc::new(FailingListener),
            Arc::new(OkListener {
                calls: calls.clone(),
            }),
        ]);

        dispatcher
            .dispatch(StateChange::DeviceCreated { id: 1 })
            .await;

        assert_eq!(*calls.lock().unwrap(), 1);
    }
}
