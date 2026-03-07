#[derive(Debug)]
pub enum GatewayEvent {
    DeviceValueObserved {
        id: u32,
        value: Option<f64>,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    DeviceCreated {
        id: u32,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    DeviceRemoved {
        id: u32,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
}
