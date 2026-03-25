FROM rust:1.88-slim AS builder
WORKDIR /app

# Copy manifests
COPY telemetry/Cargo.toml telemetry/Cargo.toml
COPY shared_models/Cargo.toml shared_models/Cargo.toml
COPY Cargo.lock Cargo.lock

# Copy sources
COPY telemetry/src telemetry/src
COPY shared_models/src shared_models/src
# COPY telemetry/config.toml telemetry/config.toml

# Build inside telemetry folder
WORKDIR /app/telemetry
RUN cargo build --release

# ---------- Runtime ----------
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/telemetry/target/release/telemetry /app/telemetry
# COPY telemetry/config.toml /config.toml

CMD ["./telemetry"]