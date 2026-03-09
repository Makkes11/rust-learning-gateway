# Rust Learning Gateway

A structured learning journey through Rust, from fundamentals to building a production-oriented Industrial IoT system.

## Repository Structure
```
rust-learning-gateway/
│
├── days/              # Daily exercises (Day 1-29: Fundamentals)
│   ├── day01/         # File I/O
│   ├── day02/         # File processing and calculations
│   ├── ...
│   └── day29/         # Advanced borrowing and lifetimes
│
├── gateway/           # Main project: Industrial IoT Gateway (Phase 2)
│   ├── src/
│   ├── tests/
│   ├── Cargo.toml
│   ├── config.toml
│   └── README.md
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

### Phase 2: Industrial IoT Gateway

Transition from exercises to building a cohesive, production-oriented system. The gateway demonstrates real-world architecture patterns: event-driven design, protocol integration, reliability considerations, and clean separation of concerns.

**→ See [gateway/README.md](gateway/README.md) for full project documentation**

---

## Quick Start

### Build & Run the Gateway
```bash
cd gateway
cargo run
```

### Run Tests
```bash
# All tests
cargo test

# Specific test file
cargo test --test api_endpoints

# With output
cargo test -- --nocapture
```

### Run a Day's Exercise
```bash
cd days/dayXX/project_name
cargo run
```

## Project Motivation

This repository documents learning Rust not through isolated tutorials, but by building a realistic system that addresses real industrial challenges. The focus is on understanding not just language features, but production-grade architecture, concurrency safety, and failure-mode thinking.

---

## Tech Stack

- **Rust** (stable)
- **Tokio** (async runtime)
- **Axum** (web framework)
- **MQTT** (rumqttc)
- **Modbus TCP** (tokio-modbus)

---

## Target Audience

- Rust learners interested in backend or systems engineering
- Industrial IoT / Edge engineers
- Technical interviewers and recruiters

---

## License

MIT License