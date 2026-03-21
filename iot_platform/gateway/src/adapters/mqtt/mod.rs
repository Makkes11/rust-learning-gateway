use crate::config::MqttConfig;
use crate::core::state::ListenerError;
use crate::core::state::{StateChange, StateListener};
use async_trait::async_trait;
use rumqttc::{AsyncClient, MqttOptions};
use shared_models::{
    DeviceContext, DeviceCreatedPayload, DeviceRemovedPayload, DeviceValueObservedPayload, Metadata,
};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::task;
use tracing::error;

struct MqttMessage {
    topic: String,
    payload: Vec<u8>,
}

pub struct MqttPublisher {
    pub config: MqttConfig,
    pub gateway_id: String,
    pub gateway_name: String,
    sender: mpsc::Sender<MqttMessage>,
}

impl MqttPublisher {
    pub async fn new(
        config: MqttConfig,
        gateway_id: String,
        gateway_name: String,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let asset_name = config.device_name.clone();
        let mut mqtt_options = MqttOptions::new(&config.client_id, &config.broker, config.port);
        mqtt_options.set_keep_alive(Duration::from_secs(5));

        let (client, mut eventloop) = AsyncClient::new(mqtt_options, 1000);

        task::spawn(async move {
            loop {
                match eventloop.poll().await {
                    Ok(_) => {}
                    Err(e) => {
                        error!("{}: MQTT eventloop error: {}", asset_name, e);
                        tokio::time::sleep(Duration::from_secs(2)).await;
                    }
                }
            }
        });

        let (tx, mut rx) = mpsc::channel::<MqttMessage>(1000);

        let client_clone = client.clone();

        let asset_name = config.device_name.clone();

        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if let Err(e) = client_clone
                    .publish(msg.topic, rumqttc::QoS::AtLeastOnce, false, msg.payload)
                    .await
                {
                    error!("{}: MQTT publish failed: {}", asset_name, e);
                }
            }
        });

        Ok(Self {
            config: config,
            gateway_id,
            gateway_name,
            sender: tx,
        })
    }
}

#[async_trait]
impl StateListener for MqttPublisher {
    async fn on_event(&self, event: StateChange) -> Result<(), ListenerError> {
        match event {
            StateChange::DeviceCreated { id, timestamp } => {
                let topic = format!("{}/devices/{}/created", self.gateway_name, id);
                let payload_model: DeviceCreatedPayload = DeviceCreatedPayload {
                    ctx: DeviceContext {
                        gateway_id: self.gateway_id.clone(),
                        gateway_name: self.gateway_name.clone(),
                        device_name: self.config.device_name.clone(),
                        device_id: id,
                    },
                    meta: Metadata {
                        timestamp: timestamp,
                    },
                };
                let payload = serde_json::to_vec(&serde_json::json!(payload_model))
                    .map_err(|e| ListenerError::Mqtt(e.to_string()))?;
                self.sender
                    .send(MqttMessage { topic, payload })
                    .await
                    .map_err(|e| ListenerError::Mqtt(e.to_string()))?;
            }
            StateChange::DeviceUpdated {
                id,
                value,
                timestamp,
            } => {
                let topic = format!("{}/devices/{}/value", self.gateway_name, id);

                let payload_model = DeviceValueObservedPayload {
                    ctx: DeviceContext {
                        gateway_id: self.gateway_id.clone(),
                        gateway_name: self.gateway_name.clone(),
                        device_name: self.config.device_name.clone(),
                        device_id: id,
                    },
                    value,
                    meta: Metadata { timestamp },
                };
                let payload = serde_json::to_vec(&serde_json::json!(payload_model))
                    .map_err(|e| ListenerError::Mqtt(e.to_string()))?;
                self.sender
                    .send(MqttMessage { topic, payload })
                    .await
                    .map_err(|e| ListenerError::Mqtt(e.to_string()))?;
            }
            StateChange::DeviceRemoved { id, timestamp } => {
                let topic = format!("{}/devices/{}/removed", self.gateway_name, id);
                let payload_model: DeviceRemovedPayload = DeviceRemovedPayload {
                    ctx: DeviceContext {
                        gateway_id: self.gateway_id.clone(),
                        gateway_name: self.gateway_name.clone(),
                        device_name: self.config.device_name.clone(),
                        device_id: id,
                    },
                    meta: Metadata {
                        timestamp: timestamp,
                    },
                };
                let payload = serde_json::to_vec(&serde_json::json!(payload_model))
                    .map_err(|e| ListenerError::Mqtt(e.to_string()))?;
                self.sender
                    .send(MqttMessage { topic, payload })
                    .await
                    .map_err(|e| ListenerError::Mqtt(e.to_string()))?;
            }
        }
        Ok(())
    }
}
