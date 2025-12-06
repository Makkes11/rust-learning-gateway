mod device;

use device::{GatewayEvent, GatewayState};
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let mut gateway = GatewayState::new();

    // initialize two devices
    gateway.apply_event(GatewayEvent::Update { id: 1, value: 0 });
    gateway.apply_event(GatewayEvent::Update { id: 2, value: 0 });

    // simulate device updates async
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    for _ in 0..5 {
        interval.tick().await;
        // Werte ändern
        gateway.apply_event(GatewayEvent::Update {
            id: 1,
            value: rand::random::<i32>() % 100,
        });
        gateway.apply_event(GatewayEvent::Update {
            id: 2,
            value: rand::random::<i32>() % 100,
        });
        println!("{:?}", gateway);
    }
}
