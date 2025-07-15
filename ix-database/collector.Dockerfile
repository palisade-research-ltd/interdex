# collector.Dockerfile

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
