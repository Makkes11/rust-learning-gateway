# Rust Learning Gateway

A structured learning journey through Rust, from fundamentals to building a production-oriented Industrial IoT system.

## Repository Structure
```
rust-learning-gateway/
│
├── days/              # Daily exercises (Day 1-29: Fundamentals)
│   ├── day01_read_file/
│   ├── day02_process_number_file/
│   └── ...
│
├── gateway/           # Main project: Industrial IoT Gateway
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

### Phase 2: Industrial IoT Gateway (Day 30+)

Transition from exercises to building a cohesive, production-oriented system. The gateway demonstrates real-world architecture patterns: event-driven design, protocol integration, reliability considerations, and clean separation of concerns.

**→ See [gateway/README.md](gateway/README.md) for full project documentation**

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

---

## Target Audience

- Rust learners interested in backend or systems engineering
- Industrial IoT / Edge engineers
- Technical interviewers and recruiters

---

## License

MIT License