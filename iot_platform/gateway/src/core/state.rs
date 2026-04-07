use crate::core::{device::Device, events::GatewayEvent};
use std::sync::Arc;
use tokio::sync::{mpsc::Sender, Mutex};
use tracing::info;

#[derive(Debug, Clone, PartialEq)]
pub enum StateChange {
    DeviceCreated {
        id: String,
        timestamp: i64,
    },
    DeviceUpdated {
        id: String,
        value: f64,
        timestamp: i64,
    },
    DeviceRemoved {
        id: String,
        timestamp: i64,
    },
}

#[async_trait::async_trait]
pub trait StateListener: Send + Sync + 'static {
    // Methods return Result to allow for centralized error reporting
    async fn on_event(&self, event: StateChange) -> Result<(), ListenerError>;
}

use std::fmt;

#[derive(Debug, Default, Clone)]
pub struct GatewayState {
    pub devices: Vec<Device>,
}

impl GatewayState {
    pub fn new() -> Self {
        Self { devices: vec![] }
    }

    pub fn apply_event(&mut self, ev: GatewayEvent) -> Option<StateChange> {
        match ev {
            GatewayEvent::DeviceValueObserved {
                id,
                value,
                timestamp,
            } => {
                let dev = match self.devices.iter_mut().find(|d| d.id == id) {
                    Some(d) => d,
                    None => {
                        self.devices.push(Device {
                            id: id.clone(),
                            value: None,
                            timestamp,
                        });
                        self.devices.last_mut().unwrap()
                    }
                };

                dev.value = Some(value);
                dev.timestamp = timestamp;
                Some(StateChange::DeviceUpdated {
                    id: id.clone(),
                    value,
                    timestamp,
                })
            }
            GatewayEvent::DeviceRemoved { id, timestamp } => {
                if let Some(pos) = self.devices.iter().position(|d| d.id == id) {
                    self.devices.remove(pos);
                } else {
                    // If device not found, we can choose to ignore or return an error. Here we ignore.
                    info!("Attempted to remove non-existent device with id {}", id);
                }

                Some(StateChange::DeviceRemoved { id, timestamp })
            }
            GatewayEvent::DeviceCreated { id, timestamp } => {
                let dev = self.devices.iter_mut().find(|d| d.id == id);

                if let Some(device) = dev {
                    device.timestamp = timestamp;

                    return Some(StateChange::DeviceCreated { id, timestamp });
                } else {
                    self.devices.push(Device {
                        id: id.clone(),
                        value: None,
                        timestamp,
                    });

                    return Some(StateChange::DeviceCreated {
                        id: id.clone(),
                        timestamp,
                    });
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
