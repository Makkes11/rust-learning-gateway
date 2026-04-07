use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone)]
pub struct Device {
    pub id: String,
    pub value: Option<f64>,
    pub timestamp: i64,
}

#[derive(Debug, Deserialize)]
pub struct DeviceInput {
    pub id: String,
    pub value: f64,
}
