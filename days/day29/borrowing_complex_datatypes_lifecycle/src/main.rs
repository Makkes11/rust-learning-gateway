fn main() {
    let dev_1 = Device { id: 1, value: 1 };
    let dev_2 = Device { id: 2, value: 2 };
    let mut state = GatewayState {
        devices: vec![dev_1, dev_2],
    };

    let ev_1 = Event::Update { id: 1, value: 100 };
    let ev_2 = Event::Update { id: 2, value: 200 };
    let ev_3 = Event::Remove(1);

    println!("{:?}", state);
    apply_event(&mut state, ev_1);
    println!("{:?}", state);
    apply_event(&mut state, ev_2);
    println!("{:?}", state);
    apply_event(&mut state, ev_3);
    println!("{:?}", state);
}

enum Event {
    Update { id: u32, value: i32 },
    Remove(u32),
}

#[derive(Debug)]
struct Device {
    id: u32,
    value: i32,
}

#[derive(Debug)]
struct GatewayState {
    devices: Vec<Device>,
}

fn find_device(state: &GatewayState, id: u32) -> Option<&Device> {
    state.devices.iter().find(|d| d.id == id)
}

fn find_device_mut(state: &mut GatewayState, id: u32) -> Option<&mut Device> {
    state.devices.iter_mut().find(|d| d.id == id)
}

fn update_device(state: &mut GatewayState, id: u32, new_value: i32) -> bool {
    match find_device_mut(state, id) {
        Some(d) => {
            d.value = new_value;
            true
        }
        None => false,
    }
}

fn apply_event(state: &mut GatewayState, ev: Event) {
    match ev {
        Event::Update { id, value } => {
            if !update_device(state, id, value) {
                state.devices.push(Device { id, value });
            }
        }
        Event::Remove(id) => {
            state.devices.retain(|d| d.id != id);
        }
    }
}
