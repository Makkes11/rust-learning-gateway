use rumqttc::{AsyncClient, MqttOptions, QoS};
use serde_json::json;
use std::time::Duration;

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
                    eprintln!("MQTT error: {err}");
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        });

        Ok(Self { client })
    }

    pub async fn publish_device_update(&self, id: u32, value: i32) {
        let topic = format!("devices/{}/value", id);

        let payload = json!({
            "id": id,
            "value": value,
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        if let Err(err) = self
            .client
            .publish(topic, QoS::AtLeastOnce, false, payload.to_string())
            .await
        {
            eprintln!("MQTT publish failed: {err}");
        }
    }

    pub async fn delete_device(&self, id: u32) {
        let topic = format!("devices/{}/deleted", id);
        let payload = json!({
            "id": id,
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        if let Err(err) = self
            .client
            .publish(topic, QoS::AtLeastOnce, false, payload.to_string())
            .await
        {
            eprintln!("MQTT publish failed: {err}");
        }
    }
}
