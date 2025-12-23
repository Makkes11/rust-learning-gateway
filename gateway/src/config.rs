use std::error::Error;

#[derive(Debug, serde::Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, serde::Deserialize)]
pub struct MqttConfig {
    pub broker: String,
    pub port: u16,
    pub client_id: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct PollingConfig {
    pub interval_ms: u64,
}

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub mqtt: MqttConfig,
    pub polling: PollingConfig,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let contents = std::fs::read_to_string("config.toml")?;
        Ok(toml::from_str(&contents)?)
    }

    pub fn load_or_default() -> Self {
        Config::load().unwrap_or_else(|_| {
            eprintln!("⚠ config.toml not found, use defaults");
            Config::default()
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "127.0.0.1".into(),
                port: 3000,
            },
            mqtt: MqttConfig {
                broker: "localhost".into(),
                port: 1883,
                client_id: "rust-gateway".into(),
            },
            polling: PollingConfig { interval_ms: 500 },
        }
    }
}
