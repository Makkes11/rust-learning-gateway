use crate::config::SimulationConfig;
use crate::core::{lifecycle::Lifecycle, state::GatewayEvent};
use async_trait::async_trait;
use tokio::{
    select,
    sync::{broadcast, mpsc::Sender},
    time::{Duration, sleep},
};
use tracing::info;

#[derive(Clone)]
pub struct SimulationPoller {
    config: SimulationConfig,
    tx: Sender<GatewayEvent>,
}

impl SimulationPoller {
    pub fn new(config: SimulationConfig, tx: Sender<GatewayEvent>) -> Self {
        Self { config, tx }
    }
}

#[async_trait]
impl Lifecycle for SimulationPoller {
    async fn run(self, shutdown: broadcast::Receiver<()>) {
        let tx = self.tx.clone();
        let mut tick_shutdown = shutdown.resubscribe();

        tokio::spawn(async move {
            loop {
                select! {
                    _ = sleep(Duration::from_millis(self.config.interval_ms)) => {
                        for device_id in 1..=2 { // Dummy snapshot: IDs 1..2, kann später dynamisch
                            let _ = tx.send(GatewayEvent::DeviceValueObserved {
                                id: device_id,
                                value: Some(rand::random::<f64>() * 100.0 + self.config.add_value as f64),
                            }).await;
                        }
                    }

                    _ = tick_shutdown.recv() => {
                        info!("Simulation task shutting down");
                        break;
                    }
                }
            }

            info!("Simulation task stopped");
        });
    }
}
