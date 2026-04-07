# Telemetry Ingestion Service

Backend service that subscribes to MQTT topics published by the gateway, processes telemetry events through a domain-driven processor, and persists time-series data into PostgreSQL.

## Overview

The telemetry service acts as the data persistence layer of the IoT platform. It consumes structured MQTT messages (published by the gateway), transforms them into domain-level intents via a deterministic processor, and executes those intents against PostgreSQL.

**Design Principle:** Ingestion → Processing → Storage. The processor is deterministic and side-effect free. Storage is responsible only for data integrity, never business logic.

## Architecture

```
MQTT Broker
    ↓
MqttAdapter (subscribes to +/devices/#)
    ↓
TelemetryService (implements TelemetryInputPort)
    ↓
DefaultProcessor (TelemetryMessage → Vec<Intent>)
    ↓
PostgresStorage (executes intents against DB)
    ↓
PostgreSQL
```

### Hexagonal Architecture (Ports & Adapters)

The service follows a strict ports & adapters pattern:

- **Ports** (traits defining boundaries):
  - `TelemetryInputPort` — entry point for incoming messages
  - `TelemetryProcessorPort` — event-to-intent transformation
  - `TelemetryStoragePort` — intent execution against storage

- **Adapters** (implementations):
  - `MqttAdapter` — MQTT subscription and message parsing
  - `PostgresStorage` — SQL execution for each intent type

- **Core**:
  - `TelemetryService` — orchestrates processor and storage
  - `DefaultProcessor` — deterministic event-to-intent mapping

## Domain Model

### Events (Input)

Consumed from MQTT via `shared_models::TelemetryMessage`:

| Event | MQTT Topic Suffix | Description |
|-------|-------------------|-------------|
| `DeviceCreated` | `/created` | Explicit device registration |
| `DeviceValueObserved` | `/value` | Telemetry measurement |
| `DeviceRemoved` | `/removed` | Device decommissioned |

### Intents (Output)

Produced by the processor, executed by storage:

| Intent | Trigger |
|--------|---------|
| `EnsureGatewayExists` | All events |
| `EnsureDeviceExists` | All events |
| `UpsertDeviceMetadata` | `DeviceCreated` |
| `RecordMeasurement` | `DeviceValueObserved` |
| `ReactivateDevice` | `DeviceValueObserved` (if previously removed) |
| `MarkDeviceRemoved` | `DeviceRemoved` |

### Device States

- **implicit** — created from telemetry only (no explicit `DeviceCreated` event)
- **active** — explicitly created or reactivated
- **removed** — logically deleted (soft delete, data preserved)

Transitions: `implicit → active` (on DeviceCreated), `removed → active` (on DeviceValueObserved)

## Database Schema

```sql
-- gateways table
CREATE TABLE gateways (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL
);

-- devices table
CREATE TABLE devices (
    id SERIAL PRIMARY KEY,
    gateway_id TEXT NOT NULL REFERENCES gateways(id),
    name TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    removed_at TIMESTAMP
);

-- device_values table (time-series, append-only)
CREATE TABLE device_values (
    device_id INT NOT NULL REFERENCES devices(id),
    timestamp TIMESTAMP NOT NULL,
    value DOUBLE PRECISION NOT NULL,
    PRIMARY KEY (device_id, timestamp)
);
```

Schema is auto-initialized via `db/init/init.sql` when using Docker.

## Source Layout

```
src/
├── adapters/
│   ├── mqtt/                # MQTT subscription and message parsing
│   └── postgres_storage/    # Intent execution against PostgreSQL
│
├── config/                  # Environment-based configuration (MqttConfig, DbConfig)
│
├── core/
│   ├── ports/               # Trait definitions (Input, Processor, Storage)
│   ├── processors/          # DefaultProcessor (event → intent mapping)
│   └── services/            # TelemetryService (orchestration)
│
├── domain/
│   └── intents.rs           # Intent enum (domain-level actions)
│
├── lib.rs
└── main.rs

docs/
├── adr/                     # Architecture Decision Records
│   ├── 001-device-auto-creation.md
│   └── 002-device-reactivation.md
└── architecture/
    ├── system-overview.md
    ├── domain-model.md
    ├── processing-model.md
    └── architecture-diagram.png
```

## Configuration

Configuration is loaded from environment variables:

| Variable | Default | Description |
|----------|---------|-------------|
| `MQTT_HOST` | `localhost` | MQTT broker hostname |
| `MQTT_PORT` | `1883` | MQTT broker port |
| `MQTT_CLIENT` | `default-client` | MQTT client ID |
| `DATABASE_URL` | — | PostgreSQL connection string (required) |
| `RUST_LOG` | `info` | Log level |

## Quick Start

### Docker (recommended)

From the workspace root:

```bash
cd iot_platform
docker-compose up --build
```

This starts the full stack: gateway, telemetry, MQTT broker, Modbus simulator, and PostgreSQL.

### Local Development

Requires a running MQTT broker and PostgreSQL instance.

```bash
export MQTT_HOST=localhost
export MQTT_PORT=1883
export DATABASE_URL=postgres://telemetry_user:telemetry_pass@localhost:5432/telemetry_db

cd iot_platform
cargo run -p telemetry
```

## Key Design Decisions

### Intent-Based Storage (ADR-001, ADR-002)

The processor never writes to storage directly. It produces a list of intents that the storage adapter executes. This keeps the processor deterministic and testable independently of infrastructure.

### Device Auto-Creation

MQTT events can arrive before an explicit `DeviceCreated` event. The system allows implicit device creation from telemetry data to avoid data loss. See [ADR-001](docs/adr/001-device-auto-creation.md).

### Device Reactivation

A removed device is automatically reactivated when new telemetry arrives. Telemetry is considered a stronger signal than lifecycle events. See [ADR-002](docs/adr/002-device-reactivation.md).

### Out-of-Order Tolerance

All events are assumed to be potentially out-of-order, duplicated, or delayed. The processor and storage layer handle these conditions gracefully using idempotent operations (upserts, conditional updates).

## Limitations

- No authentication or authorization
- No data aggregation or alerting
- No horizontal scaling
- Single MQTT subscription loop (no consumer groups)

## License

MIT License
