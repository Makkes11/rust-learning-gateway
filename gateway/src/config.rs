use std::error::Error;

#[derive(Debug, serde::Deserialize, Clone)]
pub struct ApiConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct MqttConfig {
    pub broker: String,
    pub port: u16,
    pub client_id: String,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct RegisterMapping {
    pub address: u16,
    pub count: u16,
    pub device_id: u32,
    pub scale: f64,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct ModbusConfig {
    pub host: String,
    pub port: u16,
    pub slave_id: u8,
    pub poll_interval_ms: u64,
    pub registers: Vec<RegisterMapping>,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct SimulationConfig {
    pub interval_ms: u64,
    pub add_value: i32,
}

#[derive(Debug, serde::Deserialize, PartialEq, Clone)]
pub enum SourceMode {
    Simulation,
    Modbus,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Config {
    pub mode: SourceMode,
    pub api: ApiConfig,
    pub mqtt: MqttConfig,
    pub modbus: ModbusConfig,
    pub simulation: SimulationConfig,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let contents = std::fs::read_to_string("config.toml")?;
        Ok(toml::from_str(&contents)?)
    }

    pub fn load_or_default() -> Self {
        Config::load().unwrap_or_else(|_| {
            eprintln!("⚠ config.toml not found, using defaults");
            Config::default()
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mode: SourceMode::Simulation,
            api: ApiConfig {
                host: "127.0.0.1".into(),
                port: 8080,
            },
            mqtt: MqttConfig {
                broker: "localhost".into(),
                port: 1883,
                client_id: "rust-gateway".into(),
            },
            modbus: ModbusConfig {
                host: "127.0.0.1".into(),
                port: 5020,
                slave_id: 1,
                poll_interval_ms: 1000,
                registers: vec![
                    RegisterMapping {
                        address: 0,
                        count: 1,
                        device_id: 1,
                        scale: 1.0,
                    },
                    RegisterMapping {
                        address: 5,
                        count: 2,
                        device_id: 2,
                        scale: 0.1,
                    },
                ],
            },
            simulation: SimulationConfig {
                interval_ms: 2000,
                add_value: 1,
            },
        }
    }
}
