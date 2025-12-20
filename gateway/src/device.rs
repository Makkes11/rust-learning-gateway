use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Device {
    pub id: u32,
    pub value: i32,
}

#[derive(Debug, serde::Deserialize)]
pub struct DeviceInput {
    pub id: u32,
    pub value: i32,
}
