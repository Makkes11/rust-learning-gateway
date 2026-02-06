use crate::state::{StateChange, StateListener};
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
