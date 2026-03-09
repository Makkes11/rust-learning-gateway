# ---------- Builder ----------
FROM rust:1.86-slim AS builder

WORKDIR /app

# 1. Cargo Dateien kopieren
COPY Cargo.toml Cargo.lock ./

# 2. Dummy main erzeugen (nur für Dependency Build)
RUN mkdir src && echo "fn main() {}" > src/main.rs

# 3. Dependencies kompilieren (wird gecached)
RUN cargo build --release

# 4. Jetzt echten Source Code kopieren
COPY src ./src

# 5. Final build
RUN cargo build --release


# ---------- Runtime ----------
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gateway /app/gateway

EXPOSE 8080

CMD ["./gateway"]