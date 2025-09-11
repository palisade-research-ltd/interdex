# Fixed Rust Data Collector Dockerfile
FROM debian:bullseye-slim
USER root
WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    wget \
    && rm -rf /var/lib/apt/lists/*

# Create app user for security
RUN useradd -r -u 1001 -s /bin/bash appuser

# Create necessary directories with proper permissions
RUN mkdir -p /app /app/logs /app/data /app/configs /app/scripts && \
    chown -R appuser:appuser /app

# Copy the compiled binary (make sure this exists)
COPY datacollector/build/datacollector /usr/local/bin/datacollector

# Copy configuration file
COPY datacollector/configs/datacollector_config.toml /app/configs

# Copy entrypoint file
COPY datacollector/scripts/datacollector_entrypoint.sh /app/scripts

# # Set permissions
RUN chmod +x /usr/local/bin/datacollector && \
    chown root:root /usr/local/bin/datacollector && \
    chown appuser:appuser /app/configs/datacollector_config.toml && \
    chown appuser:appuser /app/scripts/datacollector_entrypoint.sh && \
    chmod +x /app/scripts/datacollector_entrypoint.sh

# Environment variables
ENV RUST_LOG=info
ENV CLICKHOUSE_URL=http://database:8123

# Expose port if your collector serves HTTP
EXPOSE 9009

# Run the collector with startup script
CMD ["/datacollector_entrypoint.sh"]

