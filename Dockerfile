# Stage 1: build
FROM rust:1.82-slim AS builder

WORKDIR /build
COPY . .
COPY .env /app/.env

RUN apt-get update && apt-get install -y \
    iproute2 \
    pkg-config \
    libssl-dev \
 && cargo build --release

# Stage 2: runtime
FROM debian:12-slim

RUN apt-get update && apt-get install -y \
    iproute2 \
    libssl-dev \
    ca-certificates \
    libc-bin # <--- aggiunto

WORKDIR /app

# Copia il binario e il file .env
COPY --from=builder /build/target/release/agent-test /usr/local/bin/agent-test
COPY --from=builder /build/.env /app/.env

ENTRYPOINT ["agent-test"]
