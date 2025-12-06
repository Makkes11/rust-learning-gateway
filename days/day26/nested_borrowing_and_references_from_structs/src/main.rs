fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Sensor {
    id: u32,
    value: i32,
    active: bool,
}

fn describe(sensor: &Sensor) -> String {
    let Sensor { id, value, active } = *sensor;

    if !active {
        return format!("Sensor {id} inactive");
    }

    if value < 0 {
        return format!("Sensor {id} error");
    }

    return format!("Sensor {id}: {value}");
}

#[derive(Debug)]
struct Config {
    name: String,
    retries: u32,
    enabled: bool,
}

fn is_enabled(cfg: &Config) -> bool {
    let Config {
        retries: _,
        enabled,
        name: _,
    } = *cfg;

    return enabled;
}

fn take_name(cfg: Config) -> String {
    cfg.name
}

#[derive(Debug)]
struct SensorData {
    id: u32,
    value: i32,
    timestamp: u64,
}

#[derive(Debug)]
enum GatewayEvent {
    SensorUpdate(SensorData),
    ConnectionLost { id: u32, reason: String },
    Shutdown,
}

fn handle(ev: GatewayEvent) -> String {
    match ev {
        GatewayEvent::SensorUpdate(SensorData {
            id,
            value,
            timestamp,
        }) => {
            if value < 0 {
                return format!("Sensor {id} invalid");
            } else if value > 1000 {
                return format!("Sensor {id} overflow");
            } else {
                format!("Sensor {id}: {value} at {timestamp}")
            }
        }
        GatewayEvent::ConnectionLost { id, reason } => format!("Lost {id}: {reason}"),
        GatewayEvent::Shutdown => "Shutting down".into(),
    }
}
