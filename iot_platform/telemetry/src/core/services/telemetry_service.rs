use shared_models::TelemetryMessage;

use crate::core::ports::{
    telemetry_input_port::TelemetryInputPort, telemetry_processor_port::TelemetryProcessorPort,
    telemetry_storage_port::TelemetryStoragePort,
};

pub struct TelemetryService {
    pub processor: Box<dyn TelemetryProcessorPort>,
    pub storage: Box<dyn TelemetryStoragePort>,
}

#[async_trait::async_trait]
impl TelemetryInputPort for TelemetryService {
    async fn on_message(&self, msg: TelemetryMessage) {
        tracing::debug!(?msg, "Processing telemetry message");
        let intents = self.processor.process(msg);

        if let Err(e) = self.storage.execute(intents).await {
            tracing::error!(error = ?e, "Failed to execute intents");
        }
    }
}
