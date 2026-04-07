use crate::{core::ports::TelemetryProcessorPort, domain::intents::Intent};
use shared_models::TelemetryMessage::{DeviceCreated, DeviceRemoved, DeviceValueObserved};

pub struct DefaultProcessor;

impl TelemetryProcessorPort for DefaultProcessor {
    fn process(&self, msg: shared_models::TelemetryMessage) -> Vec<Intent> {
        match msg {
            DeviceValueObserved(p) => vec![
                Intent::EnsureGatewayExists {
                    gateway_id: p.ctx.gateway_id.clone(),
                    gateway_name: p.ctx.gateway_name.clone(),
                },
                Intent::EnsureDeviceExists {
                    device_id: p.ctx.device_id.clone(),
                    gateway_id: p.ctx.gateway_id.clone(),
                },
                Intent::RecordMeasurement {
                    device_id: p.ctx.device_id.clone(),
                    gateway_id: p.ctx.gateway_id.clone(),
                    value: Some(p.value),
                    timestamp: p.meta.timestamp,
                },
                Intent::ReactivateDevice {
                    device_id: p.ctx.device_id.clone(),
                    gateway_id: p.ctx.gateway_id.clone(),
                    timestamp: p.meta.timestamp,
                },
            ],
            DeviceCreated(p) => vec![
                Intent::EnsureGatewayExists {
                    gateway_id: p.ctx.gateway_id.clone(),
                    gateway_name: p.ctx.gateway_name.clone(),
                },
                Intent::EnsureDeviceExists {
                    device_id: p.ctx.device_id.clone(),
                    gateway_id: p.ctx.gateway_id.clone(),
                },
                Intent::UpsertDeviceMetadata {
                    device_id: p.ctx.device_id.clone(),
                    gateway_id: p.ctx.gateway_id.clone(),
                    gateway_name: p.ctx.gateway_name.clone(),
                    device_name: p.ctx.device_name.clone(),
                    timestamp: p.meta.timestamp,
                },
            ],
            DeviceRemoved(p) => vec![
                Intent::EnsureGatewayExists {
                    gateway_id: p.ctx.gateway_id.clone(),
                    gateway_name: p.ctx.gateway_name.clone(),
                },
                Intent::EnsureDeviceExists {
                    device_id: p.ctx.device_id.clone(),
                    gateway_id: p.ctx.gateway_id.clone(),
                },
                Intent::MarkDeviceRemoved {
                    device_id: p.ctx.device_id.clone(),
                    gateway_id: p.ctx.gateway_id.clone(),
                    timestamp: p.meta.timestamp,
                },
            ],
        }
    }
}
