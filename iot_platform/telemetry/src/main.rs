use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut mqttoptions = MqttOptions::new("subscriber-client", "localhost", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    // subscribe to topic
    client
        .subscribe("Gateway-1/devices/#", QoS::AtLeastOnce)
        .await
        .unwrap();

    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Packet::Publish(p))) => {
                println!(
                    "Received on topic {}: {}",
                    p.topic,
                    String::from_utf8_lossy(&p.payload)
                );
            }
            Ok(_) => {}
            Err(e) => {
                println!("Error: {:?}", e);
                break;
            }
        }
    }
}
