use crate::device::Device;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::Sender;

#[derive(Debug)]
pub enum GatewayEvent {
    Update { id: u32, value: i32 },
    Remove(u32),
    Tick(i32),
}

#[derive(Debug, Default)]
pub struct GatewayState {
    pub devices: Vec<Device>,
}

impl GatewayState {
    pub fn new() -> Self {
        Self { devices: vec![] }
    }

    pub fn apply_event(&mut self, ev: GatewayEvent) {
        match ev {
            GatewayEvent::Update { id, value } => {
                if let Some(dev) = self.devices.iter_mut().find(|d| d.id == id) {
                    dev.value = value;
                } else {
                    self.devices.push(Device { id, value });
                }
            }
            GatewayEvent::Remove(id) => self.devices.retain(|d| d.id != id),
            GatewayEvent::Tick(v) => {
                self.devices.iter_mut().for_each(|d| d.value += v);
            }
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub tx: Sender<GatewayEvent>,
    pub state: Arc<Mutex<GatewayState>>,
}
