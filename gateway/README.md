# Industrial IoT Gateway

Production-grade, event-driven Industrial IoT gateway written in Rust.

---

## Features

- Event-driven architecture (single-writer pattern)
- REST API (Axum)
- MQTT publishing
- Modbus TCP client
- Async runtime (Tokio)
- Graceful shutdown
- Configuration-driven
- Structured logging

---

## Architecture
```
REST API / Modbus / Background Tasks
            ↓
    mpsc::channel (Events)
            ↓
      Event Loop (Single Writer)
            ↓
    ┌───────┴────────┐
    ↓                ↓
GatewayState    Side Effects
(Arc<Mutex>)    (MQTT, Logging)
```

**Key Principle:** All state changes flow through a single event loop - no race conditions.

---

## Quick Start
```bash
cd gateway
cargo run

# With debug logging
RUST_LOG=debug cargo run

# Server starts on http://127.0.0.1:8080
```

---

## API Reference

### `GET /devices`
List all devices.
```json
[{"id": 1, "value": 42.5}]
```

### `PUT /devices/{id}`
Create device.
```json
{"id": 1, "value": null}
```

### `POST /devices`
Update device value.
```json
{"id": 1, "value": 42.5}
```

### `DELETE /devices/{id}`
Remove device. Returns `204 No Content`.

---

## MQTT Integration

### Topics
- `devices/{id}/created`
- `devices/{id}/value`
- `devices/{id}/deleted`

### Example Payload
```json
{
  "id": 1,
  "value": 42.5,
  "timestamp": "2024-12-30T12:00:00Z"
}
```

### Subscribe
```bash
mosquitto_sub -h localhost -t "devices/#" -v
```

---

## Configuration

Edit `config.toml`:
```toml
[server]
host = "127.0.0.1"
port = 8080

[mqtt]
broker = "localhost"
port = 1883
client_id = "rust-gateway"

[modbus]
enabled = true
host = "127.0.0.1"
port = 502
slave_id = 1
poll_interval_ms = 1000

[[modbus.registers]]
address = 0
count = 2           # 1=16bit, 2=32bit
device_id = 100
scale = 0.1
```

---

## Modbus TCP

Configure registers to poll from Modbus devices:
```toml
[[modbus.registers]]
address = 30775      # Register address
count = 2            # 16bit or 32bit
device_id = 200      # Gateway device ID
scale = 0.01         # Scaling factor
```

---

## Project Structure
```
gateway/
├── src/
│   ├── main.rs          # Bootstrap
│   ├── api/             # REST handlers
│   ├── state/           # Domain model
│   ├── mqtt/            # MQTT publisher
│   ├── modbus/          # Modbus client
│   ├── device.rs        # Device model
│   ├── config.rs        # Configuration
│   └── lifecycle.rs     # Service lifecycle
├── config.toml
└── Cargo.toml
```

---

## Core Concepts

### Event-Driven State
```rust
pub enum GatewayEvent {
    DeviceCreated { id: u32 },
    DeviceValueObserved { id: u32, value: Option<f64> },
    Remove(u32),
}
```

Benefits:
- Deterministic behavior
- No race conditions
- Easy debugging
- Replayable events

---

## Development
```bash
# Run tests
cargo test

# Linting
cargo clippy

# Formatting
cargo fmt
```

---

## Roadmap

**Short-Term:**
- Event handler separation
- Unit/integration tests
- Persistent event log

**Mid-Term:**
- OPC UA client
- Metrics endpoint (Prometheus)
- Docker container

**Long-Term:**
- WebSocket streaming
- Plugin architecture
- Web dashboard

---

## Why Rust?

- Memory safety without GC
- Zero-cost abstractions
- Fearless concurrency
- Modern tooling

Perfect for reliable industrial systems.

---

## License

MIT License

---

**Part of a [learning journey](../README.md) - production architecture, educational purpose.**