use std::env;

use sqlx::PgPool;

use telemetry::{
    adapters::{mqtt::MqttAdapter, postgres_storage::PostgresStorage},
    config::AppConfig,
    core::{processors::DefaultProcessor, services::TelemetryService},
};

use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()))
        .init();
    dotenv::dotenv().ok();

    let config = AppConfig::load();

    let processor = DefaultProcessor;

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;

    let storage = PostgresStorage::new(config.db, pool);

    let service = TelemetryService {
        processor: Box::new(processor),
        storage: Box::new(storage),
    };

    let mqtt = MqttAdapter {
        config: config.mqtt,
        input_port: Box::new(service),
    };

    mqtt.run().await?;

    Ok(())
}
