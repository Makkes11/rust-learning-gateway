use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone)]
pub struct Device {
    pub id: u32,
    pub value: Option<f64>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct DeviceInput {
    pub id: u32,
    pub value: f64,
}
