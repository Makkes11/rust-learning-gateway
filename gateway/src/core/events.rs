#[derive(Debug)]
pub enum GatewayEvent {
    DeviceValueObserved { id: u32, value: Option<f64> },
    DeviceCreated { id: u32 },
    DeviceRemoved { id: u32 },
}
