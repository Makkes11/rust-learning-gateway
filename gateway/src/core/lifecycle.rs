use async_trait::async_trait;
use tokio::sync::broadcast;

// trait for services that are started and shall be stopped
#[async_trait]
pub trait Lifecycle: Send + 'static {
    // starts service and runs until receiving shutdown signal
    async fn run(self, shutdown: broadcast::Receiver<()>);
}
