#[derive(Debug)]
pub enum GatewayEvent {
    DeviceValueObserved {
        id: String,
        value: f64,
        timestamp: i64,
    },
    DeviceCreated {
        id: String,
        timestamp: i64,
    },
    DeviceRemoved {
        id: String,
        timestamp: i64,
    },
}
