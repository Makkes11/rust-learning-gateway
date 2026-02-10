# Industrial IoT Gateway

Event-driven edge gateway for industrial systems with Modbus TCP integration, MQTT distribution, and REST API.

## Overview

This gateway connects industrial devices through a deterministic, event-driven architecture. It polls machine data via Modbus TCP, processes events through a single-writer loop, and exposes data via MQTT and REST API.

**Design Principle:** External inputs generate events that are processed serially. Side effects (MQTT, logging) are separated from state mutations, ensuring deterministic behavior, easy debugging, and clean extensibility.

## Problem & Solution

**Problem:** Industrial edge systems must handle unstable networks, external protocols, and concurrent data sources without producing inconsistent state or hard-to-reproduce errors.

**Solution:** Centralized event model with a single-writer pattern for state mutations. External inputs (API, Modbus, simulation) generate events that are processed serially. Side effects like MQTT publishing are clearly separated from state.

## Features

- Event-driven architecture with single-writer state pattern
- Modbus TCP polling
- MQTT publishing
- REST API for device management
- Deterministic simulation mode
- Graceful shutdown handling
- Structured logging
- Configuration-driven operation

## Architecture
```
External Adapters (API, Modbus, Simulation)
            ↓
    GatewayEvents → mpsc::channel
            ↓
      Dispatcher (Single Writer)
            ↓
       GatewayState (mutated serially)
            ↓
    StateChange Events → Listeners
            ↓
    Side Effects (MQTT, Logging)
```

**Key Components:**

- **core/** - Domain logic and state management
- **adapters/** - External interfaces (API, Modbus, MQTT, Simulation)
- **transport/** - Event transport layer
- **config.rs** - Configuration loading

## Quick Start

### Requirements

- Rust stable toolchain
- Cargo
- Optional: MQTT broker (e.g., Mosquitto)

### Installation
```bash
git clone <repository-url>
cd gateway
cargo run
```

Server starts on `http://127.0.0.1:8080`

## Configuration

Edit `config.toml`:
```toml
mode = "Modbus" # or "Simulation"

[api]
host = "127.0.0.1"
port = 8080

[mqtt]
broker = "localhost"
port = 1883
client_id = "rust-gateway"

[modbus]
host = "127.0.0.1"
port = 502
slave_id = 1
poll_interval_ms = 1000

[[modbus.registers]]
address = 0
count = 2
device_id = 100
scale = 0.1

[simulation]
interval_ms = 2000
add_value = 1
```
**Notes:**

- Data source selection is controlled via `mode`.
- Only one source (Modbus or Simulation) runs at a time.
- No runtime enable/disable flags are used.

## Source Layout
```
src/
├── adapters/            # External interfaces
│   ├── api/             # REST API (Axum)
│   ├── modbus/          # Modbus TCP poller
│   ├── mqtt/            # MQTT publisher
│   ├── simulation/      # Simulation data source
│   └── spawn_service.rs # Lifecycle task spawning
│
├── core/                # Domain and state logic
│   ├── device.rs
│   ├── dispatcher.rs
│   ├── lifecycle.rs
│   └── state.rs
│
├── logging/             # Logging listeners
├── transport/           # Event transport abstractions
├── config.rs            # Configuration model + loading
└── main.rs              # Application bootstrap
```

## API Reference

### REST Endpoints

- `GET /devices` - List all devices
- `PUT /devices/{id}` - Create device
- `POST /devices` - Update device value
- `DELETE /devices/{id}` - Remove device

### MQTT Topics

- `devices/{id}/created` - Device created
- `devices/{id}/value` - Value updated
- `devices/{id}/deleted` - Device removed

**Behavior:**

- Default QoS: 0
- No retained messages
- Publishing errors are logged but do not block state progression
- MQTT is treated as a side-effect listener, never a source of truth

### Example Usage
```bash
# Create device
curl -X PUT http://127.0.0.1:8080/devices/1

# Update value
curl -X POST http://127.0.0.1:8080/devices \
  -H "Content-Type: application/json" \
  -d '{"id":1,"value":42.5}'

# Subscribe to MQTT
mosquitto_sub -h localhost -t "devices/#" -v
```

## Key Design Decisions

### Single-Writer Event Loop

**Decision:** All state mutations flow through one event loop  
**Reason:** Eliminates race conditions and non-deterministic behavior  
**Tradeoff:** Lower parallelism for state mutations, but deterministic and debuggable

### Adapter-Based Architecture

**Decision:** Clear separation between domain logic and I/O  
**Reason:** Clean boundaries, testable core  
**Tradeoff:** More boilerplate, but better maintainability

### Deterministic Simulation

Simulation mode acts as a controlled data source for development and testing. It generates synthetic device values and feeds them into the same event pipeline as real Modbus data.

- Uses the same GatewayEvent flow as Modbus
- Intended for development, demos, and early testing
- Currently non-deterministic (randomized values)
- Deterministic seeding is planned for future CI integration tests

## Explicit Non-Goals

- Not a complete IoT platform product
- Limited protocol support (focus on architecture)
- No authentication or authorization system
- No dependency on real hardware in MVP

## Limitations

- No persistent state
- Limited protocol coverage
- No horizontal scaling

## Security Considerations

- No authentication in MVP
- No protocol-level encryption
- Intended for trusted network environments

## Performance

- Event loop limits parallel state mutations
- Suitable for edge-typical workloads
- Designed for reliability over maximum throughput

## Testing

The project currently relies on manual testing via REST, MQTT subscriptions, and simulation mode.

**Planned improvements:**

- Unit tests for core state and dispatcher
- Integration tests using simulation + local MQTT broker
- Deterministic simulation for CI reproducibility

## Deployment

- Local execution via `cargo run`
- Docker and docker-compose support planned but not yet implemented
- systemd service example planned for Linux edge deployment

## Extensibility

New adapters or listeners can be added without modifying core logic. The event-driven architecture supports clean extension points.

## Future Ideas

- Persistent event log
- Prometheus metrics endpoint
- Digital twin registry
- CI/CD pipeline

## Glossary

- **GatewayEvent** - Internal events for state changes
- **Single-Writer** - Pattern with exactly one location for state mutations
- **Adapter** - Module for connecting external systems

## License

MIT License

**Built with Rust** - Production architecture, reliability-focused design.
