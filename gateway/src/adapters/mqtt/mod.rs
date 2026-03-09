use crate::core::state::ListenerError;
use crate::core::state::{StateChange, StateListener};
use async_trait::async_trait;
use rumqttc::{AsyncClient, MqttOptions};
use std::time::Duration;
use tokio::task;
use tracing::error;

pub struct MqttPublisher {
    pub client: AsyncClient,
}

impl MqttPublisher {
    pub async fn new(
        broker: &str,
        port: u16,
        client_id: &str,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let mut mqtt_options = MqttOptions::new(client_id, broker, port);
        mqtt_options.set_keep_alive(Duration::from_secs(5));

        let (client, mut eventloop) = AsyncClient::new(mqtt_options, 10);

        // Spawn MQTT event loop in a Send-safe future
        task::spawn(async move {
            loop {
                if let Err(err) = eventloop.poll().await {
                    error!("MQTT event loop error: {}", err);
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        });

        Ok(Self { client })
    }
}

#[async_trait]
impl StateListener for MqttPublisher {
    async fn on_event(&self, event: StateChange) -> Result<(), ListenerError> {
        match event {
            StateChange::DeviceCreated { id, .. } => {
                let topic = format!("devices/{}/created", id);
                let payload = format!("{{\"id\":{}}}", id);
                self.client
                    .publish(topic, rumqttc::QoS::AtLeastOnce, false, payload)
                    .await
                    .map_err(|e| ListenerError::Mqtt(e.to_string()))?;
            }
            StateChange::DeviceUpdated { id, value, .. } => {
                let topic = format!("devices/{}/value", id);
                let payload = format!("{{\"value\":{}}}", value.unwrap_or(0.0));
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
