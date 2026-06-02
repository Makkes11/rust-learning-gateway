# Rust Learning Gateway

A structured learning journey through Rust, from fundamentals to building a production-oriented Industrial IoT platform.

## Repository Structure
```
rust-learning-gateway/
│
├── days/                    # Daily exercises (Day 1-29: Fundamentals)
│   ├── day01/               # File I/O
│   ├── day02/               # File processing and calculations
│   ├── ...
│   └── day29/               # Advanced borrowing and lifetimes
│
├── iot_platform/            # Cargo workspace: Industrial IoT Platform
│   ├── gateway/             # Edge gateway (Modbus, MQTT, REST API)
│   ├── telemetry/           # Telemetry ingestion service (MQTT → PostgreSQL)
│   ├── shared_models/       # Shared domain types across services
│   ├── docker-compose.yaml  # Full-stack deployment
│   └── Cargo.toml           # Workspace manifest
│
└── README.md
```

---

## Learning Phases

### Phase 1: Rust Fundamentals (Days 1-29)

Daily exercises focused on core Rust concepts:

- **Ownership & Borrowing** - Memory management and the borrow checker
- **Lifetimes** - Explicit lifetime annotations
- **Enums & Pattern Matching** - Type-safe data modeling
- **Error Handling** - `Result<T, E>` and `Option<T>` patterns
- **Traits** - Polymorphism and code reuse
- **Testing** - Unit and integration tests
- **State Machines** - Type-state pattern implementation

Each day is a self-contained exercise exploring a specific concept.

**→ See [days/README.md](days/README.md) for the full exercise overview**

### Phase 2: Industrial IoT Gateway

Event-driven edge gateway connecting industrial devices via Modbus TCP, distributing data over MQTT, and exposing a REST API. Demonstrates single-writer state management, adapter-based architecture, and deterministic event processing.

**→ See [iot_platform/gateway/README.md](iot_platform/gateway/README.md) for full documentation**

### Phase 3: Telemetry Ingestion Service

Backend service that subscribes to MQTT topics published by the gateway, processes telemetry events through a domain-driven processor, and persists time-series data into PostgreSQL. Implements hexagonal architecture with ports & adapters, intent-based storage, and resilient event handling.

**→ See [iot_platform/telemetry/README.md](iot_platform/telemetry/README.md) for full documentation**

---

## Quick Start

### Run the Full Platform (Docker)
```bash
cd iot_platform
docker-compose up --build
```

This starts: Gateway (port 8080), MQTT broker (port 1883), Modbus simulator (port 5020), PostgreSQL (port 5432), and Telemetry service.

### Run Individual Services
```bash
cd iot_platform

# Gateway only
cargo run -p gateway

# Telemetry only (requires MQTT broker + PostgreSQL)
cargo run -p telemetry
```

### Run Tests
```bash
cd iot_platform

# All workspace tests
cargo test

# Specific package
cargo test -p gateway

# Specific test file
cargo test -p gateway --test api_endpoints

# With output
cargo test -- --nocapture
```

### Run a Day's Exercise
```bash
cd days/dayXX/project_name
cargo run
```

---

## Project Motivation

This repository documents learning Rust not through isolated tutorials, but by building a realistic system that addresses real industrial challenges. The focus is on understanding not just language features, but production-grade architecture, concurrency safety, and failure-mode thinking.

---

## Tech Stack

- **Rust** (stable)
- **Tokio** (async runtime)
- **Axum** (web framework)
- **MQTT** (rumqttc)
- **Modbus TCP** (tokio-modbus)
- **PostgreSQL** (sqlx)
- **Docker / Docker Compose**

---

## Target Audience

- Rust learners interested in backend or systems engineering
- Industrial IoT / Edge engineers
- Technical interviewers and recruiters

---

## License

MIT License
