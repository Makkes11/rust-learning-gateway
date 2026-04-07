use crate::domain::intents::Intent;
use async_trait::async_trait;

#[async_trait]
pub trait TelemetryStoragePort: Send + Sync {
    async fn execute(
        &self,
        intents: Vec<Intent>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
