use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Device {
    pub id: u32,
    pub value: Option<f64>,
}

#[derive(Debug, serde::Deserialize)]
pub struct DeviceInput {
    pub id: u32,
    pub value: f64,
}
