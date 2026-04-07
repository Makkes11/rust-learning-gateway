use async_trait::async_trait;
use shared_models::TelemetryMessage;

#[async_trait]
pub trait TelemetryInputPort: Send + Sync {
    async fn on_message(&self, msg: TelemetryMessage);
}
