use async_trait::async_trait;
use chrono::{DateTime, NaiveDateTime, Utc};
use tracing::{error, info};

use crate::{
    config::DbConfig, core::ports::telemetry_storage_port::TelemetryStoragePort,
    domain::intents::Intent,
};

pub struct PostgresStorage {
    pub config: DbConfig,
    pub pool: sqlx::Pool<sqlx::Postgres>,
}

impl PostgresStorage {
    pub fn new(config: DbConfig, pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        PostgresStorage { config, pool }
    }

    pub fn ts_to_datetime(ts_millis: i64) -> NaiveDateTime {
        let secs = ts_millis / 1000;
        let nsecs = (ts_millis % 1000) * 1_000_000;

        DateTime::<Utc>::from_timestamp(secs, nsecs as u32)
            .expect("Invalid timestamp")
            .naive_utc()
    }
}

#[async_trait]
impl TelemetryStoragePort for PostgresStorage {
    async fn execute(
        &self,
        intents: Vec<Intent>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        for intent in &intents {
            match intent {
                // Ensure that the gateway exists in the DB
                Intent::EnsureGatewayExists {
                    gateway_id,
                    gateway_name,
                } => {
                    let res = sqlx::query(
                        r#"
                        INSERT INTO gateways (id, name)
                        VALUES ($1, $2)
                        ON CONFLICT (id) DO UPDATE SET name = EXCLUDED.name;
                        "#,
                    )
                    .bind(gateway_id)
                    .bind(gateway_name)
                    .execute(&self.pool)
                    .await;

                    match res {
                        Ok(r) => info!(
                            "Inserted/Updated gateway {} (rows affected: {})",
                            gateway_id,
                            r.rows_affected()
                        ),
                        Err(e) => error!("Failed to insert/update gateway {}: {:?}", gateway_id, e),
                    }
                }

                // Ensure that the device exists in the DB
                Intent::EnsureDeviceExists {
                    device_id,
                    gateway_id,
                } => {
                    let res = sqlx::query(
                        r#"
                        INSERT INTO devices (gateway_id, id)
                        VALUES ($1, $2)
                        ON CONFLICT (gateway_id, id) DO NOTHING;
                        "#,
                    )
                    .bind(gateway_id)
                    .bind(device_id)
                    .execute(&self.pool)
                    .await;

                    match res {
                        Ok(r) => info!(
                            "Ensured device {} exists (rows affected: {})",
                            device_id,
                            r.rows_affected()
                        ),
                        Err(e) => error!("Failed to ensure device {} exists: {:?}", device_id, e),
                    }
                }

                // Record a telemetry measurement
                Intent::RecordMeasurement {
                    device_id,
                    gateway_id,
                    value,
                    timestamp,
                } => {
                    let ts = Self::ts_to_datetime(*timestamp);
                    let res = sqlx::query(
                        r#"
                        INSERT INTO device_values (gateway_id, device_id, "timestamp", value)
                        VALUES ($1, $2, $3, $4)
                        ON CONFLICT (gateway_id, device_id, "timestamp") DO UPDATE SET value = EXCLUDED.value;
                        "#,
                    )
                    .bind(gateway_id)
                    .bind(device_id)
                    .bind(ts)
                    .bind(value)
                    .execute(&self.pool)
                    .await;

                    match res {
                        Ok(_) => info!("Recorded measurement for device {}", device_id),
                        Err(e) => error!(
                            "Failed to record measurement for device {}: {:?}",
                            device_id, e
                        ),
                    }
                }

                // Update last_seen_at only if the new timestamp is later
                Intent::UpdateDeviceLastSeen {
                    device_id,
                    gateway_id,
                    timestamp,
                } => {
                    let ts = Self::ts_to_datetime(*timestamp);
                    let res = sqlx::query(
                        r#"
                        UPDATE devices
                        SET last_seen = GREATEST(COALESCE(last_seen, $3), $3)
                        WHERE id = $1 AND gateway_id = $2;
                        "#,
                    )
                    .bind(device_id)
                    .bind(gateway_id)
                    .bind(ts)
                    .execute(&self.pool)
                    .await;

                    match res {
                        Ok(r) => info!(
                            "Updated last seen for device {} (rows affected: {})",
                            device_id,
                            r.rows_affected()
                        ),
                        Err(e) => error!(
                            "Failed to update last seen for device {}: {:?}",
                            device_id, e
                        ),
                    }
                }

                // Reactivate a device by clearing removed_at
                Intent::ReactivateDevice {
                    device_id,
                    gateway_id,
                    timestamp,
                } => {
                    let ts = Self::ts_to_datetime(*timestamp);
                    let res = sqlx::query(
                        r#"
                        UPDATE devices
                        SET removed_at = NULL
                        WHERE id = $1 AND gateway_id = $2
                        AND (removed_at IS NOT NULL AND removed_at < $3);
                        "#,
                    )
                    .bind(device_id)
                    .bind(gateway_id)
                    .bind(ts)
                    .execute(&self.pool)
                    .await;

                    match res {
                        Ok(r) => info!(
                            "Reactivated device {} (rows affected: {})",
                            device_id,
                            r.rows_affected()
                        ),
                        Err(e) => error!("Failed to reactivate device {}: {:?}", device_id, e),
                    }
                }

                // Mark device as removed
                Intent::MarkDeviceRemoved {
                    device_id,
                    gateway_id,
                    timestamp,
                } => {
                    let ts = Self::ts_to_datetime(*timestamp);
                    let res = sqlx::query(
                        r#"
                        UPDATE devices
                        SET removed_at = $3
                        WHERE id = $1 AND gateway_id = $2
                        AND (removed_at IS NULL OR removed_at < $3);
                        "#,
                    )
                    .bind(device_id)
                    .bind(gateway_id)
                    .bind(ts)
                    .execute(&self.pool)
                    .await;

                    match res {
                        Ok(r) => info!(
                            "Marked device {} as removed (rows affected: {})",
                            device_id,
                            r.rows_affected()
                        ),
                        Err(e) => error!("Failed to mark device {} as removed: {:?}", device_id, e),
                    }
                }

                // Update device and gateway metadata
                Intent::UpsertDeviceMetadata {
                    device_id,
                    gateway_id,
                    gateway_name,
                    device_name,
                    timestamp,
                } => {
                    let ts = Self::ts_to_datetime(*timestamp);

                    // Update device metadata
                    let res = sqlx::query(
                        r#"
                        UPDATE devices
                        SET 
                            name = $3,
                            last_seen = GREATEST(COALESCE(last_seen, $4), $4),
                            removed_at = CASE WHEN removed_at IS NOT NULL THEN NULL ELSE removed_at END,
                            created_at = COALESCE(created_at, $4)
                        WHERE id = $1 AND gateway_id = $2;
                        "#
                    )
                    .bind(device_id)
                    .bind(gateway_id)
                    .bind(device_name)
                    .bind(ts)
                    .execute(&self.pool)
                    .await;

                    match res {
                        Ok(r) => info!(
                            "Upserted device metadata {} (rows affected: {})",
                            device_id,
                            r.rows_affected()
                        ),
                        Err(e) => error!("Failed to upsert device metadata {}: {:?}", device_id, e),
                    }

                    // Update gateway metadata
                    let res = sqlx::query(
                        r#"
                        UPDATE gateways
                        SET name = $2
                        WHERE id = $1;
                        "#,
                    )
                    .bind(gateway_id)
                    .bind(gateway_name)
                    .execute(&self.pool)
                    .await;

                    match res {
                        Ok(r) => info!(
                            "Upserted gateway metadata {} (rows affected: {})",
                            gateway_id,
                            r.rows_affected()
                        ),
                        Err(e) => {
                            error!("Failed to upsert gateway metadata {}: {:?}", gateway_id, e)
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
