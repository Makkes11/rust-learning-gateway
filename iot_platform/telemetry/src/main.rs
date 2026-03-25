use chrono::{DateTime, NaiveDateTime, Utc};
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use shared_models::{DeviceCreatedPayload, DeviceRemovedPayload, DeviceValueObservedPayload};
use sqlx::PgPool;
use std::env;
use std::time::Duration;
use tracing::{debug, error, info};
use tracing_subscriber;

fn ts_to_datetime(ts_millis: i64) -> NaiveDateTime {
    let secs = ts_millis / 1000;
    let nsecs = (ts_millis % 1000) * 1_000_000;

    DateTime::<Utc>::from_timestamp(secs, nsecs as u32)
        .expect("Invalid timestamp")
        .naive_utc()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up logging
    tracing_subscriber::fmt()
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()))
        .init();
    dotenv::dotenv().ok();

    let mqtt_host = env::var("MQTT_Host").unwrap_or_else(|_| "localhost".to_string());
    let mqtt_port = env::var("MQTT_Port")
        .unwrap_or_else(|_| "1883".to_string())
        .parse::<u16>()
        .expect("MQTT_Port must be a valid u16");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;

    let mut mqttoptions = MqttOptions::new("subscriber-client", &mqtt_host, mqtt_port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    client
        .subscribe("Gateway-1/devices/#", QoS::AtLeastOnce)
        .await?;

    info!("Telemetry service started and subscribed to MQTT topic: Gateway-1/devices/#");

    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Packet::Publish(p))) => {
                let topic = p.topic.clone();
                let payload = &p.payload;

                // =========================
                // CREATE
                // =========================
                if topic.ends_with("/created") {
                    let msg: DeviceCreatedPayload = serde_json::from_slice(payload)?;
                    let ts = ts_to_datetime(msg.meta.timestamp);

                    // 1. Gateway upsert
                    //todo: use macro query! when possible
                    let res: Result<sqlx::postgres::PgQueryResult, sqlx::Error> = sqlx::query(
                        r#"
                        INSERT INTO gateways (id, name)
                        VALUES ($1, $2)
                        ON CONFLICT (id) DO UPDATE
                        SET name = EXCLUDED.name
                        "#,
                    )
                    .bind(&msg.ctx.gateway_id)
                    .bind(&msg.ctx.gateway_name)
                    .execute(&pool)
                    .await;

                    match res {
                        Ok(result) => {
                            if result.rows_affected() == 0 {
                                debug!("Gateway {} already up-to-date", msg.ctx.gateway_id);
                            } else {
                                info!("Inserted/Updated gateway {}", msg.ctx.gateway_id);
                            }
                        }
                        Err(e) => {
                            error!(
                                "Failed to insert/update gateway {}: {:?}",
                                msg.ctx.gateway_id, e
                            );
                        }
                    }

                    // 2. Device insert
                    //todo: use macro query! when possible
                    let res: Result<sqlx::postgres::PgQueryResult, sqlx::Error> = sqlx::query(
                        r#"
                        INSERT INTO devices (id, gateway_id, name, created_at)
                        VALUES ($1, $2, $3, $4)
                        ON CONFLICT (id) DO NOTHING
                        "#,
                    )
                    .bind(msg.ctx.device_id as i32)
                    .bind(&msg.ctx.gateway_id)
                    .bind(&msg.ctx.device_name)
                    .bind(ts)
                    .execute(&pool)
                    .await;

                    match res {
                        Ok(result) => {
                            if result.rows_affected() == 0 {
                                debug!("Device {} already up-to-date", msg.ctx.device_id);
                            } else {
                                info!("Inserted/Updated device {}", msg.ctx.device_id);
                            }
                        }
                        Err(e) => {
                            error!(
                                "Failed to insert/update device {}: {:?}",
                                msg.ctx.device_id, e
                            );
                        }
                    }
                }
                // =========================
                // VALUE
                // =========================
                else if topic.ends_with("/value") {
                    let msg: DeviceValueObservedPayload = serde_json::from_slice(payload)?;
                    let ts = ts_to_datetime(msg.meta.timestamp);

                    // 1. try to insert gateway if it doesn't exist (created event might have been missed)
                    //todo: use macro query! when possible
                    let res: Result<sqlx::postgres::PgQueryResult, sqlx::Error> = sqlx::query(
                        r#"
                        INSERT INTO gateways (id, name)
                        VALUES ($1, $2)
                        ON CONFLICT (id) DO UPDATE
                        SET name = EXCLUDED.name
                        "#,
                    )
                    .bind(&msg.ctx.gateway_id)
                    .bind(&msg.ctx.gateway_name)
                    .execute(&pool)
                    .await;

                    match res {
                        Ok(result) => {
                            if result.rows_affected() == 0 {
                                debug!("Gateway {} already up-to-date", msg.ctx.gateway_id);
                            } else {
                                info!("Inserted/Updated gateway {}", msg.ctx.gateway_id);
                            }
                        }
                        Err(e) => {
                            error!(
                                "Failed to insert/update gateway {}: {:?}",
                                msg.ctx.gateway_id, e
                            );
                        }
                    }

                    // 2. try to insert device if it doesn't exist (created event might have been missed)
                    //todo: use macro query! when possible
                    let res: Result<sqlx::postgres::PgQueryResult, sqlx::Error> = sqlx::query(
                        r#"
                        INSERT INTO devices (id, gateway_id, name, created_at)
                        VALUES ($1, $2, $3, NOW())
                        ON CONFLICT (id) DO NOTHING
                        "#,
                    )
                    .bind(msg.ctx.device_id as i32)
                    .bind(msg.ctx.gateway_id)
                    .bind(msg.ctx.device_name)
                    .execute(&pool)
                    .await;

                    match res {
                        Ok(result) => {
                            if result.rows_affected() == 0 {
                                debug!("Device {} already up-to-date", msg.ctx.device_id);
                            } else {
                                info!("Inserted/Updated device {}", msg.ctx.device_id);
                            }
                        }
                        Err(e) => {
                            error!(
                                "Failed to insert/update device {}: {:?}",
                                msg.ctx.device_id, e
                            );
                        }
                    }

                    // 3. insert or update device value
                    //todo: use macro query! when possible
                    let res: Result<sqlx::postgres::PgQueryResult, sqlx::Error> = sqlx::query(
                        r#"
                        INSERT INTO device_values (device_id, timestamp, value)
                        VALUES ($1, $2, $3)
                        ON CONFLICT (device_id, timestamp)
                        DO UPDATE SET value = EXCLUDED.value
                        "#,
                    )
                    .bind(msg.ctx.device_id as i32)
                    .bind(ts)
                    .bind(msg.value)
                    .execute(&pool)
                    .await;

                    match res {
                        Ok(result) => {
                            if result.rows_affected() == 0 {
                                debug!(
                                    "Device value for device {} at timestamp {} already up-to-date",
                                    msg.ctx.device_id, ts
                                );
                            } else {
                                info!(
                                    "Inserted/Updated device value for device {} at timestamp {}",
                                    msg.ctx.device_id, ts
                                );
                            }
                        }
                        Err(e) => {
                            error!(
                                "Failed to insert/update device value for device {} at timestamp {}: {:?}",
                                msg.ctx.device_id, ts, e
                            );
                        }
                    }
                }
                // =========================
                // REMOVE
                // =========================
                else if topic.ends_with("/removed") {
                    let msg: DeviceRemovedPayload = serde_json::from_slice(payload)?;
                    let ts = ts_to_datetime(msg.meta.timestamp);

                    // 1. update device as removed
                    //todo: use macro query! when possible
                    let res: Result<sqlx::postgres::PgQueryResult, sqlx::Error> = sqlx::query(
                        r#"
                        UPDATE devices
                        SET removed_at = $1
                        WHERE id = $2
                        "#,
                    )
                    .bind(ts)
                    .bind(msg.ctx.device_id as i32)
                    .execute(&pool)
                    .await;

                    match res {
                        Ok(result) => {
                            if result.rows_affected() == 0 {
                                debug!("Device {} already marked as removed", msg.ctx.device_id);
                            } else {
                                info!("Marked device {} as removed", msg.ctx.device_id);
                            }
                        }
                        Err(e) => {
                            error!(
                                "Failed to mark device {} as removed: {:?}",
                                msg.ctx.device_id, e
                            );
                        }
                    }
                }
            }

            Ok(_) => {}

            Err(e) => {
                eprintln!("MQTT Error: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}
