use crate::config::MqttConfig;
use crate::core::state::ListenerError;
use crate::core::state::{StateChange, StateListener};
use async_trait::async_trait;
use rumqttc::{AsyncClient, MqttOptions};
use std::time::Duration;
use tokio::task;
use tracing::error;

pub struct MqttPublisher {
    pub config: MqttConfig,
    pub client: AsyncClient,
}

impl MqttPublisher {
    pub async fn new(config: MqttConfig) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let asset_name = config.device_name.clone();
        let mut mqtt_options = MqttOptions::new(&config.client_id, &config.broker, config.port);
        mqtt_options.set_keep_alive(Duration::from_secs(5));

        let (client, mut eventloop) = AsyncClient::new(mqtt_options, 10);

        // Spawn MQTT event loop in a Send-safe future
        task::spawn(async move {
            loop {
                if let Err(err) = eventloop.poll().await {
                    error!("{}: MQTT event loop error: {}", asset_name, err);
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        });

        Ok(Self {
            config: config,
            client,
        })
    }
}

#[async_trait]
impl StateListener for MqttPublisher {
    async fn on_event(&self, event: StateChange) -> Result<(), ListenerError> {
        match event {
            StateChange::DeviceCreated { id, .. } => {
                let topic = format!("{}/devices/{}/created", self.config.device_name, id);
                let payload = serde_json::to_vec(&serde_json::json!({
                    "id": id
                }))
                .map_err(|e| ListenerError::Mqtt(e.to_string()))?;
                self.client
                    .publish(topic, rumqttc::QoS::AtLeastOnce, false, payload)
                    .await
                    .map_err(|e| ListenerError::Mqtt(e.to_string()))?;
            }
            StateChange::DeviceUpdated { id, value, .. } => {
                let topic = format!("{}/devices/{}/value", self.config.device_name, id);
                let payload = serde_json::to_vec(&serde_json::json!({ "value": value }))
                    .map_err(|e| ListenerError::Mqtt(e.to_string()))?;
                self.client
                    .publish(topic, rumqttc::QoS::AtLeastOnce, false, payload)
                    .await
                    .map_err(|e| ListenerError::Mqtt(e.to_string()))?;
            }
            _ => {}
        }
        Ok(())
    }
}
