# collector.Dockerfile
#FROM rust:1.75 as builder
# COPY Cargo.toml ./
# COPY src/ ./src/
# RUN cargo build --release --bin collector

FROM debian:bullseye-slim
WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    wget \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/collector /usr/local/bin/collector

ENV RUST_LOG=info
WORKDIR /app

CMD ["collector"]
