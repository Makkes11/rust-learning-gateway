fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Device {
    id: u32,
    status: Status,
}

#[derive(Debug)]
enum Status {
    Online { load: u32 },
    Offline,
    Error(String),
}

fn status_msg(dev: &Device) -> String {
    let Device { id, status } = dev;

    match status {
        Status::Online { load } if *load < 50 => format!("Device {id}: OK"),
        Status::Online { .. } => format!("Device {id}: HIGH LOAD"),
        Status::Offline => format!("Device {id}: OFFLINE"),
        Status::Error(msg) => format!("Device {id}: ERROR {msg}"),
    }
}

// fn status_msg(dev: &Device) -> String {
//     let Device { id, status } = dev;

//     match status {
//         Status::Online { load } => match *load {
//             0..=49 => format!("Device {id}: OK"),
//             _ => format!("Device {id}: HIGH LOAD"),
//         },
//         Status::Offline => format!("Device {id}: OFFLINE"),
//         Status::Error(msg) => format!("Device {id}: ERROR {msg}"),
//     }
// }

fn reset_if_error(dev: &mut Device) {
    if let Status::Error(_) = dev.status {
        dev.status = Status::Offline;
    }
}

enum Response {
    Ok(Data),
    Timeout,
    NetworkErr(String),
}

struct Data {
    value: Option<i32>,
}

fn interpret(resp: Response) -> &'static str {
    match resp {
        Response::Timeout => "timeout",
        Response::NetworkErr(_) => "network error",
        Response::Ok(Data { value: None }) => "no data",
        Response::Ok(Data { value: Some(v) }) if v < 0 => "invalid",
        Response::Ok(Data { value: Some(v) }) if v > 0 && v <= 100 => "normal",
        Response::Ok(Data { value: Some(..) }) => "overflow",
    }
}

// fn interpret(resp: Response) -> &'static str {
//     match resp {
//         Response::Timeout => "timeout",
//         Response::NetworkErr(_) => "network error",
//         Response::Ok(Data { value: None }) => "no data",
//         Response::Ok(Data { value: Some(value) }) => match value {
//             v if v < 0 => "invalid",
//             v if v > 0 && v <= 100 => "normal",
//             _ => "overflow",
//         },
//     }
// }

enum GatewayEvent {
    Sensor { id: u32, value: i32 },
    Command(String),
    Disconnect { reason: Option<String> },
}

// fn summarize(ev: &GatewayEvent) -> String {
//     match ev {
//         GatewayEvent::Sensor { id, value } if *value < 0 => format!("sensor {id} invalid"),
//         GatewayEvent::Sensor { id, value } if *value > 1000 => format!("sensor {id} overflow"),
//         GatewayEvent::Sensor { id, value } => format!("sensor {id}: {value}"),
//         GatewayEvent::Command(s) if s.is_empty() => format!("empty command"),
//         GatewayEvent::Command(r) if r == "restart" => format!("restarting"),
//         GatewayEvent::Command(s) => format!("cmd: {s}"),
//         GatewayEvent::Disconnect { reason: Some(r) } => format!("disconnect: {r}"),
//         GatewayEvent::Disconnect { reason: None } => format!("disconnect"),
//     }
// }

fn summarize(ev: &GatewayEvent) -> String {
    match ev {
        GatewayEvent::Sensor { id, value } => match *value {
            v if v < 0 => format!("sensor {id} invalid"),
            v if v > 1000 => format!("sensor {id} overflow"),
            v => format!("sensor {id}: {v}"),
        },

        GatewayEvent::Command(cmd) => match cmd.as_str() {
            "restart" => "restarting".into(),
            "" => "empty command".into(),
            other => format!("cmd: {other}"),
        },

        GatewayEvent::Disconnect { reason } => match reason {
            Some(r) => format!("disconnect: {r}"),
            None => "disconnect".into(),
        },
    }
}
