use crate::config::ModbusConfig;
use crate::state::GatewayEvent;
use anyhow::Result;
use tokio::sync::mpsc::Sender;
use tokio::time::{Duration, sleep};
use tokio_modbus::client::Context;
use tokio_modbus::prelude::*;
use tracing::{debug, error, info};

pub struct ModbusPoller {
    config: ModbusConfig,
    tx: Sender<GatewayEvent>,
}

impl ModbusPoller {
    pub fn new(config: ModbusConfig, tx: Sender<GatewayEvent>) -> Self {
        Self { config, tx }
    }

    pub async fn start(self) {
        if !self.config.enabled {
            info!("Modbus polling disabled in config");
            return;
        }

        info!(
            "Starting Modbus poller: {}:{}",
            self.config.host, self.config.port
        );

        loop {
            match self.run_polling_loop().await {
                Ok(_) => {
                    info!("Modbus connection closed gracefully");
                }
                Err(e) => {
                    error!("Modbus connection failed: {}", e);
                    info!("Retrying in 5 seconds...");
                    sleep(Duration::from_secs(5)).await;
                }
            }
        }
    }

    async fn run_polling_loop(&self) -> Result<()> {
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

            sleep(Duration::from_millis(self.config.poll_interval_ms)).await;
        }
    }

    async fn poll_once(&self, ctx: &mut Context) -> Result<()> {
        // read holding registers (address 0, count 10)
        let registers = ctx.read_holding_registers(0, 10).await??;
        debug!("Modbus read {} registers", registers.len());

        for (index, &value) in registers.iter().enumerate() {
            let device_id = (index as u32) + 100;

            self.tx
                .send(GatewayEvent::Update {
                    id: device_id,
                    value: value as i32,
                })
                .await?;
        }

        Ok(())
    }
}
