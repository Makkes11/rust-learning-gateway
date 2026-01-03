use crate::device::Device;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::Sender;

#[derive(Debug)]
pub enum GatewayEvent {
    Update { id: u32, value: i32 },
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
            GatewayEvent::Update { id, value } => {
                if let Some(dev) = self.devices.iter_mut().find(|d| d.id == id) {
                    dev.value = value;
                } else {
                    return Err(StateError::DeviceNotFound(id));
                    // self.devices.push(Device { id, value });
                }
            }
            GatewayEvent::Remove(id) => {
                if let Some(pos) = self.devices.iter().position(|d| d.id == id) {
                    self.devices.remove(pos);
                } else {
                    return Err(StateError::DeviceNotFound(id));
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
