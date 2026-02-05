use crate::state::{ListenerError, StateChange, StateListener};
use rumqttc::{AsyncClient, MqttOptions, QoS};
use serde_json::json;
use std::time::Duration;
use tracing::{debug, error};

pub struct MqttPublisher {
    client: AsyncClient,
}

impl MqttPublisher {
    pub async fn new(broker: &str, port: u16, client_id: &str) -> Result<Self, String> {
        let mut mqtt_options = MqttOptions::new(client_id, broker, port);
        mqtt_options.set_keep_alive(Duration::from_secs(5));

        let (client, mut eventloop) = AsyncClient::new(mqtt_options, 10);

        tokio::spawn(async move {
            loop {
                if let Err(err) = eventloop.poll().await {
                    error!("MQTT event loop error: {}", err);
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        });

        Result::Ok(Self { client })
    }

    pub async fn publish_device_update(&self, id: u32, value: f64) -> Result<(), ListenerError> {
        let topic = format!("devices/{}/value", id);

        debug!("Publishing to MQTT: topic={}, value={}", topic, value);

        // timestamp is generated, not the state change timestamp
        let payload = json!({
            "id": id,
            "value": value,
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        self.send_to_mqtt(topic, payload).await
    }

    pub async fn delete_device(&self, id: u32) -> Result<(), ListenerError> {
        let topic = format!("devices/{}/deleted", id);

        debug!("Publishing to MQTT: topic={}, id={}", topic, id);

        // timestamp is generated, not the state change timestamp
        let payload = json!({
            "id": id,
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        self.send_to_mqtt(topic, payload).await
    }

    pub async fn create_device(&self, id: u32) -> Result<(), ListenerError> {
        let topic = format!("devices/{}/created", id);

        debug!("Publishing to MQTT: topic={}, id={}", topic, id);

        // timestamp is generated, not the state change timestamp
        let payload = json!({
            "id": id,
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        self.send_to_mqtt(topic, payload).await
    }

    async fn send_to_mqtt(
        &self,
        topic: String,
        payload: serde_json::Value,
    ) -> Result<(), ListenerError> {
        // 1. Convert payload to string
        let payload_str = payload.to_string();

        let result = self
            .client
            .publish(topic, QoS::AtLeastOnce, false, payload_str)
            .await;

        match result {
            Err(e) => Err(ListenerError::Mqtt(e.to_string())),
            Ok(_) => Ok(()),
        }
    }
}

#[async_trait::async_trait]
impl StateListener for MqttPublisher {
    async fn on_event(&self, event: StateChange) -> Result<(), ListenerError> {
        match event {
            StateChange::DeviceCreated { id } => self.create_device(id).await,
            StateChange::DeviceUpdated { id, value } => {
                // assumed value change, but not guaranteed
                // listener should be idempotent to repeated updates
                if let Some(val) = value {
                    self.publish_device_update(id, val).await
                } else {
                    Ok(())
                }
            }
            StateChange::DeviceRemoved { id } => self.delete_device(id).await,
        }
    }
}
