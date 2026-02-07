use crate::core::state::{ListenerError, StateChange, StateListener};
use async_trait::async_trait;
use tracing::info;

pub struct ConsoleLogger;

impl ConsoleLogger {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl StateListener for ConsoleLogger {
    async fn on_event(&self, event: StateChange) -> Result<(), ListenerError> {
        match event {
            StateChange::DeviceCreated { id } => {
                if id == 0 {
                    return Err(ListenerError::General(
                        "Device ID 0 is reserved/invalid".into(),
                    ));
                }
                info!("Device {id} was created");
            }
            StateChange::DeviceUpdated { id, value } => {
                info!("Device {id} was updated with value {:?}", value);
            }
            StateChange::DeviceRemoved { id } => info!("Device {id} was removed"),
        }

        Ok(())
    }
}
