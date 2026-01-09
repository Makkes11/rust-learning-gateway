# Rust Learning Gateway

Learning Rust by building a production-grade Industrial IoT gateway.

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
│   ├── src/
│   └── README.md
│
└── README.md
```

**→ [Gateway Documentation](gateway/README.md)**

---

## Learning Path

### Phase 1: Fundamentals (Days 1-29)

Focused exercises on core Rust concepts:

- Ownership, borrowing & lifetimes
- Pattern matching & enums
- Error handling (`Result`, `Option`)
- Traits & generics
- Testing & CLI tools
- State machines
- Complex data structures

### Phase 2: Production System (Day 30+)

Industrial IoT gateway with production-grade architecture:

- Event-driven design (single-writer pattern)
- REST API (Axum)
- Async runtime (Tokio)
- MQTT integration
- Modbus TCP client
- Configuration system
- Structured logging

---

## Tech Stack

- **Rust** (stable)
- **Tokio** (async runtime)
- **Axum** (web framework)
- **MQTT** (rumqttc)
- **Modbus TCP** (tokio-modbus)
- **Serde** (serialization)

---

## Getting Started

See [gateway/README.md](gateway/README.md) for:
- Quick start guide
- API documentation
- Configuration options
- Architecture details

---

## License

MIT License