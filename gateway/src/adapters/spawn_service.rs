use crate::core::lifecycle::Lifecycle;
use tokio::sync::broadcast;

pub fn spawn_service<T: Lifecycle + Send + 'static>(service: T, shutdown: broadcast::Receiver<()>) {
    tokio::spawn(async move {
        service.run(shutdown).await;
    });
}
