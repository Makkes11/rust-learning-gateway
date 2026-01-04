use crate::config::ModbusConfig;
use crate::lifecycle::Lifecycle;
use crate::state::GatewayEvent;
use anyhow::Result;
use async_trait::async_trait;
use tokio::select;
use tokio::sync::broadcast;
use tokio::sync::mpsc::Sender;
use tokio::time::{Duration, sleep};
use tokio_modbus::client::Context;
use tokio_modbus::prelude::*;
use tracing::{debug, error, info, warn};

pub struct ModbusPoller {
    config: ModbusConfig,
    tx: Sender<GatewayEvent>,
}

impl ModbusPoller {
    pub fn new(config: ModbusConfig, tx: Sender<GatewayEvent>) -> Self {
        Self { config, tx }
    }

    // pub async fn start(self, mut shutdown: broadcast::Receiver<()>) {}

    async fn run_polling_loop(&self, shutdown: &mut broadcast::Receiver<()>) -> Result<()> {
        let socket_addr = format!("{}:{}", self.config.host, self.config.port);

        info!("Connecting to Modbus device at {}", socket_addr);
        let mut ctx = tcp::connect_slave(socket_addr.parse()?, Slave(self.config.slave_id)).await?;

        info!("Modbus connected successfully");

        // polling loop with existing connection
        loop {
            match self.poll_once(&mut ctx).await {
                Ok(_) => {
                    debug!("Modbus poll successful");
                }
                Err(e) => {
                    error!("Modbus poll failed: {}", e);
                    return Err(e); // connection damaged, reconnect
                }
            }

            // wait either for sleep or shutdown
            select! {
                _ = sleep(Duration::from_millis(self.config.poll_interval_ms)) => {
                    // Sleep fertig → weiter pollen
                }
                _ = shutdown.recv() => {
                    info!("Modbus received shutdown signal");
                    return Ok(());  // Sauber beenden
                }
            }
        }
    }

    async fn poll_once(&self, ctx: &mut Context) -> Result<()> {
        for mapping in &self.config.registers {
            let registers = ctx
                .read_holding_registers(mapping.address, mapping.count)
                .await??;

            let raw_value: i32 = match mapping.count {
                1 => {
                    // Ein Register (16-bit) direkt als i32
                    registers[0] as i32
                }
                2 => {
                    // Zwei Register zu 32-bit kombinieren
                    let high = registers[0] as u32;
                    let low = registers[1] as u32;
                    let combined = (high << 16) | low;
                    combined as i32
                }
                _ => {
                    warn!("Unsupported register count: {}, skipping", mapping.count);
                    continue;
                }
            };

            let scaled_value = raw_value as f64 * mapping.scale;

            self.tx
                .send(GatewayEvent::DeviceValueObserved {
                    id: mapping.device_id,
                    value: Some(scaled_value),
                })
                .await?;

            debug!(
                "Device {}: address={}, raw={}, scaled={}",
                mapping.device_id, mapping.address, raw_value, scaled_value
            );
        }

        Ok(())
    }
}

#[async_trait]
impl Lifecycle for ModbusPoller {
    async fn run(self, mut shutdown: broadcast::Receiver<()>) {
        if !self.config.enabled {
            info!("Modbus polling disabled in config");
            return;
        }

        info!(
            "Starting Modbus poller: {}:{}",
            self.config.host, self.config.port
        );

        loop {
            // Check vor Reconnect ob Shutdown kam
            if shutdown.try_recv().is_ok() {
                info!("Modbus shutting down before reconnect");
                break;
            }

            match self.run_polling_loop(&mut shutdown).await {
                Ok(_) => {
                    info!("Modbus connection closed gracefully");
                    break;
                }
                Err(e) => {
                    error!("Modbus connection failed: {}", e);
                    info!("Retrying in 5 seconds...");

                    // Sleep MIT Shutdown-Check
                    select! {
                        _ = sleep(Duration::from_secs(5)) => {}
                        _ = shutdown.recv() => {
                            info!("Shutdown during reconnect wait");
                            break;
                        }
                    }
                }
            }
        }

        info!("Modbus poller stopped");
    }
}
