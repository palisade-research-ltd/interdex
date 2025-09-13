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
COPY datacollector /usr/local/bin/datacollector
COPY datacollector_config.toml /app

# Set permissions
RUN chmod +x /usr/local/bin/datacollector && \
    chown root:root /usr/local/bin/datacollector && \
    chown appuser:appuser /app/datacollector_config.toml

# Switch to app user
USER appuser

# Environment variables
ENV RUST_LOG=info
ENV CLICKHOUSE_URL=http://database:8123

# Expose port
EXPOSE 9009

# Run the application
CMD ["/usr/local/bin/datacollector"]

