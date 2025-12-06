#[derive(Debug)]
pub struct Device {
    pub id: u32,
    pub value: i32,
}

#[derive(Debug)]
pub enum GatewayEvent {
    Update { id: u32, value: i32 },
    Remove(u32),
}

#[derive(Debug)]
pub struct GatewayState {
    pub devices: Vec<Device>,
}

impl GatewayState {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
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
        }
    }
}
