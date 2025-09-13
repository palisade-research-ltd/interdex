
FROM --platform=linux/amd64 rust:1.85 as builder

WORKDIR /app

# Copy source files
COPY . . 

# Install OpenSSL development libraries
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Build inside Linux container (no cross-compilation needed!)
RUN cargo build --bin datacollector --release # --target x86_64-unknown-linux-gnu

FROM scratch as output
COPY --from=builder /app/target/release/datacollector /datacollector

