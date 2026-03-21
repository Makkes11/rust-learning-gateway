use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceContext {
    pub gateway_id: String,
    pub gateway_name: String,
    pub device_name: String,
    pub device_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceCreatedPayload {
    pub ctx: DeviceContext,
    pub meta: Metadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceValueObservedPayload {
    pub ctx: DeviceContext,
    pub value: Option<f64>,
    pub meta: Metadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceRemovedPayload {
    pub ctx: DeviceContext,
    pub meta: Metadata,
}
