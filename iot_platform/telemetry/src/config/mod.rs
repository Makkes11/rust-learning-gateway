use std::env::var;

pub struct MqttConfig {
    pub client_id: String,
    pub mqtt_host: String,
    pub mqtt_port: u16,
}

pub struct DbConfig {
    pub url: String,
}

pub struct AppConfig {
    pub mqtt: MqttConfig,
    pub db: DbConfig,
}

impl AppConfig {
    pub fn load() -> Self {
        Self {
            mqtt: MqttConfig {
                client_id: var("MQTT_CLIENT").unwrap_or_else(|_| "default-client".to_string()),
                mqtt_host: var("MQTT_HOST").unwrap_or_else(|_| "localhost".to_string()),
                mqtt_port: var("MQTT_PORT")
                    .unwrap_or_else(|_| "1883".to_string())
                    .parse::<u16>()
                    .expect("MQTT_PORT must be a valid u16"),
            },
            db: DbConfig {
                url: var("DB_URL").unwrap_or_else(|_| "postgres://localhost/telemetry".to_string()),
            },
        }
    }
}
