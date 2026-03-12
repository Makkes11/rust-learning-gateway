use crate::config::ModbusConfig;
use crate::core::{events::GatewayEvent, lifecycle::Lifecycle};
use anyhow::Result;
use async_trait::async_trait;
use std::net::SocketAddr;
use tokio::net::lookup_host;
use tokio::{
    select,
    sync::{broadcast, mpsc::Sender},
    time::{sleep, Duration},
};
use tokio_modbus::client::tcp;
use tokio_modbus::prelude::*;
use tracing::{debug, error, info, warn};

#[derive(Clone)]
pub struct ModbusPoller {
    config: ModbusConfig,
    tx: Sender<GatewayEvent>,
}

impl ModbusPoller {
    pub fn new(config: ModbusConfig, tx: Sender<GatewayEvent>) -> Self {
        Self { config, tx }
    }

    async fn poll_once(&self, ctx: &mut tokio_modbus::client::Context) -> Result<()> {
        for mapping in &self.config.registers {
            let registers = ctx
                .read_holding_registers(mapping.address, mapping.count)
                .await??;

            let raw_value: i32 = match mapping.count {
                1 => registers[0] as i32,
                2 => {
                    let high = registers[0] as u32;
                    let low = registers[1] as u32;
                    ((high << 16) | low) as i32
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
                    timestamp: chrono::Utc::now().timestamp_millis(),
                })
                .await?;

            debug!(
                "Device {}: address={}, raw={}, scaled={}",
                mapping.device_id, mapping.address, raw_value, scaled_value
            );
        }

        Ok(())
    }

    async fn run_polling_loop(
        &self,
        shutdown: &mut broadcast::Receiver<()>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let addr = format!("{}:{}", self.config.host, self.config.port);
        let mut addrs = lookup_host(addr).await?;
        let socket_addr: SocketAddr = addrs.next().ok_or_else(|| "No address found".to_string())?;

        info!(
            "{}: Connecting to Modbus device at {}",
            self.config.device_name, socket_addr
        );

        let mut ctx = tcp::connect_slave(socket_addr, Slave(self.config.slave_id)).await?;

        info!("{}: Modbus connected", self.config.device_name);

        loop {
            match self.poll_once(&mut ctx).await {
                Ok(_) => debug!("{}: Modbus poll successful", self.config.device_name),
                Err(e) => {
                    error!(
                        "{}: Modbus poll failed: {}, retrying...",
                        self.config.device_name, e
                    );
                    sleep(Duration::from_secs(1)).await;
                    continue;
                }
            }

            select! {
                _ = sleep(Duration::from_millis(self.config.poll_interval_ms)) => {},
                _ = shutdown.recv() => {
                    info!("{}: Modbus received shutdown signal", self.config.device_name);
                    return Ok(());
                }
            }
        }
    }
}

#[async_trait]
impl Lifecycle for ModbusPoller {
    async fn run(self, mut shutdown: broadcast::Receiver<()>) {
        info!(
            "Starting Modbus poller: {}:{}",
            self.config.host, self.config.port
        );

        loop {
            if shutdown.try_recv().is_ok() {
                info!(
                    "{}: Modbus shutting down before reconnect",
                    self.config.device_name
                );
                break;
            }

            match self.run_polling_loop(&mut shutdown).await {
                Ok(_) => {
                    info!(
                        "{}: Modbus connection closed gracefully",
                        self.config.device_name
                    );
                    break;
                }
                Err(e) => {
                    error!(
                        "{}: Modbus connection failed: {}",
                        self.config.device_name, e
                    );
                    info!("{}: Retrying in 5 seconds...", self.config.device_name);
                    select! {
                        _ = sleep(Duration::from_secs(5)) => {},
                        _ = shutdown.recv() => {
                            info!("{}: Shutdown during reconnect wait", self.config.device_name);
                            break;
                        }
                    }
                }
            }
        }

        info!("Modbus poller stopped");
    }
}
