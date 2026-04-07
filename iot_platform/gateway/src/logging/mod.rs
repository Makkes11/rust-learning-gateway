use crate::core::state::{ListenerError, StateChange, StateListener};
use async_trait::async_trait;
use tracing::info;

pub struct ConsoleLogger {
    device_name: String,
}

impl ConsoleLogger {
    pub fn new(device_name: &str) -> Self {
        Self {
            device_name: device_name.into(),
        }
    }
}

#[async_trait]
impl StateListener for ConsoleLogger {
    async fn on_event(&self, event: StateChange) -> Result<(), ListenerError> {
        match event {
            StateChange::DeviceCreated { id, .. } => {
                info!("{}: Device {id} was created", self.device_name);
            }
            StateChange::DeviceUpdated {
                id,
                value,
                timestamp,
            } => {
                info!(
                    "{}: Device {id} was updated with value {:?} at timestamp {}",
                    self.device_name, value, timestamp
                );
            }
            StateChange::DeviceRemoved { id, .. } => {
                info!("{}: Device {id} was removed", self.device_name);
            }
        }

        Ok(())
    }
}
