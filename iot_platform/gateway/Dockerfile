FROM rust:1.88-slim AS builder
WORKDIR /app

# Copy manifests
COPY gateway/Cargo.toml gateway/Cargo.toml
COPY shared_models/Cargo.toml shared_models/Cargo.toml
COPY Cargo.lock Cargo.lock

# Copy sources
COPY gateway/src gateway/src
COPY shared_models/src shared_models/src
COPY gateway/config.toml gateway/config.toml

# Build inside gateway folder
WORKDIR /app/gateway
RUN cargo build --release

# ---------- Runtime ----------
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update \
    && apt-get install -y ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/gateway/target/release/gateway /app/gateway
COPY gateway/config.toml /config.toml

EXPOSE 8080
CMD ["./gateway"]