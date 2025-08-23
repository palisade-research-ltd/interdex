FROM clickhouse/clickhouse-server:latest

# Remove default password file that may override configuration
RUN rm -f /etc/clickhouse-server/users.d/default-password.xml

# Create required directories
RUN mkdir -p /var/lib/clickhouse/format_schemas && \
    mkdir -p /var/lib/clickhouse/access && \
    mkdir -p /var/lib/clickhouse/user_files && \
    mkdir -p /var/lib/clickhouse/tmp && \
    mkdir -p /var/log/clickhouse-server && \
    chown -R clickhouse:clickhouse /var/lib/clickhouse && \
    chown -R clickhouse:clickhouse /var/log/clickhouse-server

# Copy configuration files
COPY clickhouse/config.xml /etc/clickhouse-server/config.xml
COPY clickhouse/users.xml /etc/clickhouse-server/users.xml

# Set proper permissions
RUN chown -R clickhouse:clickhouse /etc/clickhouse-server/ && \
    chmod 644 /etc/clickhouse-server/config.xml && \
    chmod 644 /etc/clickhouse-server/users.xml

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=30s --retries=3 \
   CMD wget --spider -q localhost:8123/ping || exit 1

EXPOSE 8123 9000

# Custom entrypoint to ensure clean configuration
COPY entrypoint.sh /custom-entrypoint.sh
RUN chmod +x /custom-entrypoint.sh

ENTRYPOINT ["/custom-entrypoint.sh"]
