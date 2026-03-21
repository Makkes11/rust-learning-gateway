use crate::core::state::{ListenerError, StateChange, StateListener};
use async_trait::async_trait;
use tracing::info;

pub struct ConsoleLogger {
    gateway_name: String,
}

impl ConsoleLogger {
    pub fn new(gateway_name: &str) -> Self {
        Self {
            gateway_name: gateway_name.into(),
        }
    }
}

#[async_trait]
impl StateListener for ConsoleLogger {
    async fn on_event(&self, event: StateChange) -> Result<(), ListenerError> {
        match event {
            StateChange::DeviceCreated { id, .. } => {
                if id == 0 {
                    return Err(ListenerError::General(
                        "Device ID 0 is reserved/invalid".into(),
                    ));
                }
                info!("{}: Device {id} was created", self.gateway_name);
            }
            StateChange::DeviceUpdated {
                id,
                value,
                timestamp,
            } => {
                info!(
                    "{}: Device {id} was updated with value {:?} at timestamp {}",
                    self.gateway_name, value, timestamp
                );
            }
            StateChange::DeviceRemoved { id, .. } => {
                info!("{}: Device {id} was removed", self.gateway_name);
            }
        }

        Ok(())
    }
}
