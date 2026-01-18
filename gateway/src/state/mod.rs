use crate::device::Device;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::Sender;
use tracing::info;

#[derive(Debug)]
pub enum GatewayEvent {
    DeviceValueObserved { id: u32, value: Option<f64> },
    DeviceCreated { id: u32 },
    Remove(u32),
}

#[derive(Debug, Clone)]
pub enum StateChange {
    DeviceCreated { id: u32 },
    DeviceUpdated { id: u32, value: Option<f64> },
    DeviceRemoved { id: u32 },
}

#[async_trait::async_trait]
pub trait StateListener: Send + Sync + 'static {
    async fn on_event(&self, event: StateChange);
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

#[derive(Debug, Default)]
pub struct GatewayState {
    pub devices: Vec<Device>,
}

impl GatewayState {
    pub fn new() -> Self {
        Self { devices: vec![] }
    }

    pub fn apply_event(&mut self, ev: GatewayEvent) -> Result<Option<StateChange>, StateError> {
        match ev {
            GatewayEvent::DeviceValueObserved { id, value } => {
                let dev = self
                    .devices
                    .iter_mut()
                    .find(|d| d.id == id)
                    .ok_or(StateError::DeviceNotFound(id))?;

                dev.value = value;
                Ok(Some(StateChange::DeviceUpdated { id, value }))
            }
            GatewayEvent::Remove(id) => {
                let pos = self
                    .devices
                    .iter()
                    .position(|d| d.id == id)
                    .ok_or(StateError::DeviceNotFound(id))?;

                self.devices.remove(pos);
                Ok(Some(StateChange::DeviceRemoved { id }))
            }
            GatewayEvent::DeviceCreated { id } => {
                let dev = self.devices.iter().find(|d| d.id == id);

                if let Some(device) = dev {
                    info!("Device with id {} already exists", device.id);
                    return Ok(None);
                } else {
                    self.devices.push(Device { id, value: None });
                    info!("Created Device with id {}", id);
                    Ok(Some(StateChange::DeviceCreated { id }))
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

pub struct Dispatcher {
    listeners: Vec<Arc<dyn StateListener>>,
}

impl Dispatcher {
    pub fn new(listeners: Vec<Arc<dyn StateListener>>) -> Self {
        Self { listeners }
    }

    pub fn dispatch(&self, event: StateChange) {
        for listener in &self.listeners {
            let listener_arc = Arc::clone(listener);
            let event_clone = event.clone();

            tokio::spawn(async move {
                listener_arc.on_event(event_clone).await;
            });
        }
    }
}
