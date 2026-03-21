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
- Modbus TCP polling with DNS resolution support
- MQTT publishing with simplified payload handling
- REST API for device management
- Deterministic simulation mode
- Docker containerization with multi-service setup
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

### Docker Setup

For containerized deployment with integrated MQTT broker and Modbus simulator:

```bash
# Build and run with docker-compose
docker-compose up --build

# Or build image manually
docker build -t rust-gateway .
docker run -p 8080:8080 rust-gateway
```

**Services included:**
- **Gateway**: Main application on port 8080
- **MQTT Broker** (Eclipse Mosquitto): Port 1883
- **Modbus Simulator**: Port 5020

**Access:**
- API: `http://localhost:8080`
- MQTT: `localhost:1883`
- Modbus: `localhost:5020`

## Configuration

Edit `config.toml`:
```toml
mode = "Modbus" # or "Simulation"

[api]
host = "0.0.0.0"  # Use 0.0.0.0 for containerized deployments
port = 8080

[mqtt]
broker = "mqtt"  # Use "mqtt" for docker-compose, "localhost" for local
port = 1883
client_id = "rust-gateway"

[modbus]
host = "modbus-sim"  # Use "modbus-sim" for docker-compose, "127.0.0.1" for local
port = 5020
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
- `POST /devices` - Create device with value
- `PUT /devices/{id}` - Update device value
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
# Create device with value
curl -X POST http://127.0.0.1:8080/devices \
  -H "Content-Type: application/json" \
  -d '{"id":1,"value":12.34}'

# Get all devices
curl http://127.0.0.1:8080/devices

# Update device value
curl -X PUT http://127.0.0.1:8080/devices/1 \
  -H "Content-Type: application/json" \
  -d '{"id":1,"value":42.5}'

# Delete device
curl -X DELETE http://127.0.0.1:8080/devices/1

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

The project includes both unit tests in the source code and integration tests for API endpoints.

**Running tests:**
```bash
cargo test
```

**Test coverage:**
- `src/core/state_tests.rs` - Unit tests for event application and state mutations
- `src/core/dispatcher_tests.rs` - Dispatcher event processing tests
- `tests/api_endpoints.rs` - Integration tests for REST API (create, read, update, delete)

**Current test results:**
- Core state management: 5 tests
- API endpoint integration: 4 test scenarios (create, read, update, delete)

**Example test scenario:**
The `api_endpoints_work` test validates the complete lifecycle:
1. Create a device via `POST /devices` with value
2. Retrieve device via `GET /devices` and verify value
3. Update device via `PUT /devices/{id}` with new value
4. Delete device via `DELETE /devices/{id}` and verify removal

**Planned improvements:**
- Performance testing under sustained load
- Chaos testing for network failures
- MQTT listener integration tests
- Deterministic simulation for CI reproducibility

## Deployment

- **Local execution:** `cargo run` or `RUST_LOG=debug cargo run`
- **Docker container:** `docker build -t rust-gateway . && docker run -p 8080:8080 rust-gateway`
- **Full stack with docker-compose:** `docker-compose up --build` (includes MQTT broker and Modbus simulator)
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
