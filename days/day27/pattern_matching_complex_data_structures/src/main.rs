fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Metrics {
    temp: Option<i32>,
    humidity: Option<u8>,
}

fn classify(m: &Metrics) -> &'static str {
    match m {
        Metrics { temp: None, .. } => "no temp data",
        Metrics { temp: Some(t), .. } if *t < 0 => "freezing",
        Metrics { temp: Some(t), .. } if *t > 60 => "overheat",
        Metrics {
            temp: Some(_),
            humidity: Some(h),
        } if *h > 80 => "humid",
        _ => "ok",
    }
}

enum Connection {
    Wifi { ssid: String, strength: i32 },
    Ethernet { speed: u32 },
    Bluetooth(Option<String>),
}

fn describe(c: Connection) -> String {
    match c {
        Connection::Wifi { ssid, strength } if strength < -80 => format!("wifi weak ({ssid})"),
        Connection::Wifi { .. } => "wifi ok".into(),

        Connection::Ethernet { speed } if speed >= 1000 => "gb ethernet".into(),
        Connection::Ethernet { .. } => "ethernet".into(),
        Connection::Bluetooth(b) => match b {
            Some(b) => format!("bt: {b}"),
            None => "bt: unknown".into(),
        },
    }
}

#[derive(Debug)]
struct Packet {
    id: u32,
    data: Option<Vec<u8>>,
}

enum GatewayMsg {
    Data(Packet),
    Heartbeat { ts: u64 },
    Error(String),
}

// fn handle(msg: GatewayMsg) -> String {
//     match msg {
//         GatewayMsg::Data(Packet { id: _, data: None }) => "empty packet".into(),
//         GatewayMsg::Data(Packet {
//             id: 0,
//             data: Some(_),
//         }) => "invalid id".into(),
//         GatewayMsg::Data(Packet {
//             id: _,
//             data: Some(v),
//         }) if v.len() == 0 => "no bytes".into(),
//         GatewayMsg::Data(Packet { id, data: Some(v) }) => {
//             let len = v.len();
//             format!("packet {id}: {len} bytes")
//         }
//         GatewayMsg::Heartbeat { ts } => format!("hb {ts}"),
//         GatewayMsg::Error(s) => format!("err: {s}"),
//     }
// }

fn handle(msg: GatewayMsg) -> String {
    match msg {
        GatewayMsg::Data(Packet { data: None, .. }) => "empty packet".into(),
        GatewayMsg::Data(Packet {
            id: 0,
            data: Some(_),
        }) => "invalid id".into(),
        GatewayMsg::Data(Packet { id, data: Some(v) }) => {
            if v.is_empty() {
                "no bytes".into()
            } else {
                format!("packet {id}: {} bytes", v.len())
            }
        }

        GatewayMsg::Heartbeat { ts } => format!("hb {ts}"),
        GatewayMsg::Error(s) => format!("err: {s}"),
    }
}
