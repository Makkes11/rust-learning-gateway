mod device;

use device::{GatewayEvent, GatewayState};

fn main() {
    let mut gateway = GatewayState::new();

    gateway.apply_event(GatewayEvent::Update { id: 1, value: 100 });
    gateway.apply_event(GatewayEvent::Update { id: 2, value: 200 });
    gateway.apply_event(GatewayEvent::Remove(1));

    println!("{:?}", gateway);
}
