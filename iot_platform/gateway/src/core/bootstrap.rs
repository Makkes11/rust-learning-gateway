use tokio::sync::mpsc::Sender;

use crate::{
    config::{Config, SourceMode},
    core::events::GatewayEvent,
};

pub async fn initialize_devices(tx: &Sender<GatewayEvent>, config: &Config) {
    match config.mode {
        SourceMode::Modbus => {
            for register in &config.modbus.registers {
                let _ = tx
                    .send(GatewayEvent::DeviceCreated {
                        id: register.device_id,
                        timestamp: chrono::Utc::now().timestamp_millis(),
                    })
                    .await;
            }
        }
        SourceMode::Simulation => {
            for device_id in 1..=2 {
                let _ = tx
                    .send(GatewayEvent::DeviceCreated {
                        id: device_id,
                        timestamp: chrono::Utc::now().timestamp_millis(),
                    })
                    .await;
                let _ = tx
                    .send(GatewayEvent::DeviceValueObserved {
                        id: device_id,
                        value: rand::random::<f64>() * 100.0,
                        timestamp: chrono::Utc::now().timestamp_millis(),
                    })
                    .await;
            }
        }
    }
}
