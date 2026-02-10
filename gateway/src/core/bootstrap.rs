use tokio::sync::mpsc::Sender;

use crate::{
    config::{Config, SourceMode},
    core::state::GatewayEvent,
};

pub async fn initialize_devices(tx: &Sender<GatewayEvent>, config: &Config) {
    match config.mode {
        SourceMode::Modbus => {
            for register in &config.modbus.registers {
                let _ = tx
                    .send(GatewayEvent::DeviceCreated {
                        id: register.device_id,
                    })
                    .await;
            }
        }
        SourceMode::Simulation => {
            for device_id in 1..=2 {
                let _ = tx.send(GatewayEvent::DeviceCreated { id: device_id }).await;
                let _ = tx
                    .send(GatewayEvent::DeviceValueObserved {
                        id: device_id,
                        value: Some(rand::random::<f64>() * 100.0),
                    })
                    .await;
            }
        }
    }
}
