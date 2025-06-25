
FROM rust:1.78-slim AS builder

WORKDIR /build

COPY . .

RUN apt-get update && apt-get install -y iproute2 && cargo build --release

FROM debian:12-slim
RUN apt-get update && apt-get install -y iproute2

COPY --from=builder /build/target/release/agent-test /usr/local/bin/agent-test

ENTRYPOINT ["agent-test"]
