use crate::core::{device::Device, events::GatewayEvent};
use std::sync::Arc;
use tokio::sync::{mpsc::Sender, Mutex};
use tracing::info;

#[derive(Debug, Clone, PartialEq)]
pub enum StateChange {
    DeviceCreated {
        id: u32,
        timestamp: i64,
    },
    DeviceUpdated {
        id: u32,
        value: Option<f64>,
        timestamp: i64,
    },
    DeviceRemoved {
        id: u32,
        timestamp: i64,
    },
}

#[async_trait::async_trait]
pub trait StateListener: Send + Sync + 'static {
    // Methods now return Result to allow for centralized error reporting
    async fn on_event(&self, event: StateChange) -> Result<(), ListenerError>;
}

#[derive(Debug)]
pub enum StateError {
    DeviceNotFound(u32),
}

use std::fmt;

impl fmt::Display for StateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StateError::DeviceNotFound(id) => {
                write!(f, "Device with id {} not found", id)
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct GatewayState {
    pub devices: Vec<Device>,
}

impl GatewayState {
    pub fn new() -> Self {
        Self { devices: vec![] }
    }

    pub fn apply_event(&mut self, ev: GatewayEvent) -> Result<Option<StateChange>, StateError> {
        match ev {
            GatewayEvent::DeviceValueObserved {
                id,
                value,
                timestamp,
            } => {
                let dev = self
                    .devices
                    .iter_mut()
                    .find(|d| d.id == id)
                    .ok_or(StateError::DeviceNotFound(id))?;

                dev.value = value;
                dev.timestamp = timestamp;
                Ok(Some(StateChange::DeviceUpdated {
                    id,
                    value,
                    timestamp,
                }))
            }
            GatewayEvent::DeviceRemoved { id, timestamp } => {
                let pos = self
                    .devices
                    .iter()
                    .position(|d| d.id == id)
                    .ok_or(StateError::DeviceNotFound(id))?;

                self.devices.remove(pos);
                Ok(Some(StateChange::DeviceRemoved { id, timestamp }))
            }
            GatewayEvent::DeviceCreated { id, timestamp } => {
                let dev = self.devices.iter().find(|d| d.id == id);

                if let Some(device) = dev {
                    info!("Device with id {} already exists", device.id);
                    return Ok(None);
                } else {
                    self.devices.push(Device {
                        id,
                        value: None,
                        timestamp: timestamp,
                    });
                    info!("Created Device with id {}", id);
                    Ok(Some(StateChange::DeviceCreated { id, timestamp }))
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub tx: Sender<GatewayEvent>,
    pub state: Arc<Mutex<GatewayState>>,
}

#[derive(Debug)]
pub enum ListenerError {
    Mqtt(String),
    General(String),
}

impl fmt::Display for ListenerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            ListenerError::Mqtt(e) | ListenerError::General(e) => e,
        };

        write!(f, "{msg}")
    }
}
