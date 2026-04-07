use crate::domain::intents::Intent;
use shared_models::TelemetryMessage;

pub trait TelemetryProcessorPort: Send + Sync {
    fn process(&self, message: TelemetryMessage) -> Vec<Intent>;
}
