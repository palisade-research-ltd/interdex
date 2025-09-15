FROM --platform=linux/amd64 debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    wget \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -u 1001 -s /bin/bash appuser

# Create directories
RUN mkdir -p /app /app/logs && \
    chown -R appuser:appuser /app

WORKDIR /app

# Copy built binary from builder stage
COPY dataproducer /usr/local/bin/dataproducer
COPY dataproducer_config.toml /app

# Set permissions
RUN chmod +x /usr/local/bin/dataproducer && \
    chown root:root /usr/local/bin/dataproducer && \
    chown appuser:appuser /app/dataproducer_config.toml

# Switch to app user
USER appuser

# Environment variables
ENV RUST_LOG=info
ENV CLICKHOUSE_URL=http://database:8123

# Expose port
EXPOSE 9009

# Run the application
CMD ["/usr/local/bin/dataproducer"]

