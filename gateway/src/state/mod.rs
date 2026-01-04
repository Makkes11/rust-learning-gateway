use crate::device::Device;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::Sender;
use tracing::{info, warn};

#[derive(Debug)]
pub enum GatewayEvent {
    DeviceValueObserved { id: u32, value: Option<f64> },
    DeviceCreated { id: u32 },
    Remove(u32),
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

    pub fn apply_event(&mut self, ev: GatewayEvent) -> Result<(), StateError> {
        match ev {
            GatewayEvent::DeviceValueObserved { id, value } => {
                if let Some(dev) = self.devices.iter_mut().find(|d| d.id == id) {
                    dev.value = value;
                } else {
                    return Err(StateError::DeviceNotFound(id));
                }
            }
            GatewayEvent::Remove(id) => {
                if let Some(pos) = self.devices.iter().position(|d| d.id == id) {
                    self.devices.remove(pos);
                } else {
                    return Err(StateError::DeviceNotFound(id));
                }
            }
            GatewayEvent::DeviceCreated { id } => {
                if let Some(device) = self.devices.iter_mut().find(|d| d.id == id) {
                    warn!("Device with id {} already exists", device.id);
                } else {
                    self.devices.push(Device { id, value: None });
                    info!("Created Device with id {}", id);
                }
            }
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct AppState {
    pub tx: Sender<GatewayEvent>,
    pub state: Arc<Mutex<GatewayState>>,
}
