use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use shared_models::{
    DeviceCreatedPayload, DeviceRemovedPayload, DeviceValueObservedPayload, TelemetryMessage,
};
use std::{error::Error, time::Duration};
use tracing::info;

use crate::{config::MqttConfig, core::ports::telemetry_input_port::TelemetryInputPort};

pub struct MqttAdapter {
    pub config: MqttConfig,
    pub input_port: Box<dyn TelemetryInputPort>,
}

impl MqttAdapter {
    pub async fn new(
        config: MqttConfig,
        input_port: Box<dyn TelemetryInputPort>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        Ok(MqttAdapter { config, input_port })
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut mqttoptions = MqttOptions::new(
            &self.config.client_id,
            &self.config.mqtt_host,
            self.config.mqtt_port,
        );

        mqttoptions.set_keep_alive(Duration::from_secs(5));

        let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

        client.subscribe("+/devices/#", QoS::AtLeastOnce).await?;

        loop {
            match eventloop.poll().await {
                Ok(Event::Incoming(Packet::Publish(p))) => {
                    let payload = &p.payload;

                    info!(
                        "Received MQTT message on topic: {} and payload: {}",
                        p.topic,
                        String::from_utf8_lossy(payload)
                    );

                    match self.parse_telemetry_message(&p.topic, &p.payload) {
                        Ok(message) => self.input_port.on_message(message).await,
                        Err(e) => eprintln!("Failed to parse telemetry message: {:?}", e),
                    }
                }
                Ok(_) => {}
                Err(e) => {
                    eprintln!("MQTT Error: {:?}", e);
                    break;
                }
            }
        }

        Ok(())
    }

    pub fn parse_telemetry_message(
        &self,
        topic: &str,
        payload_bytes: &[u8],
    ) -> Result<TelemetryMessage, Box<dyn Error>> {
        if topic.ends_with("/created") {
            let payload: DeviceCreatedPayload = serde_json::from_slice(payload_bytes)?;
            Ok(TelemetryMessage::DeviceCreated(payload))
        } else if topic.ends_with("/value") {
            let payload: DeviceValueObservedPayload = serde_json::from_slice(payload_bytes)?;
            Ok(TelemetryMessage::DeviceValueObserved(payload))
        } else if topic.ends_with("/removed") {
            let payload: DeviceRemovedPayload = serde_json::from_slice(payload_bytes)?;
            Ok(TelemetryMessage::DeviceRemoved(payload))
        } else {
            Err(format!("Unknown MQTT topic: {}", topic).into())
        }
    }
}
