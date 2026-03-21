#[derive(Debug)]
pub enum GatewayEvent {
    DeviceValueObserved {
        id: u32,
        value: Option<f64>,
        timestamp: i64,
    },
    DeviceCreated {
        id: u32,
        timestamp: i64,
    },
    DeviceRemoved {
        id: u32,
        timestamp: i64,
    },
}
