FROM clickhouse/clickhouse-server:latest

# Remove default password file that may override configuration
RUN rm -f /etc/clickhouse-server/users.d/default-password.xml

# Create required directories with proper permissions
RUN mkdir -p /var/lib/clickhouse/format_schemas && \
    mkdir -p /var/lib/clickhouse/access && \
    mkdir -p /var/lib/clickhouse/user_files && \
    mkdir -p /var/lib/clickhouse/tmp && \
    mkdir -p /var/log/clickhouse-server && \
    chown -R clickhouse:clickhouse /var/lib/clickhouse && \
    chown -R clickhouse:clickhouse /var/log/clickhouse-server

# Copy configuration files
COPY database/configs/config.xml /etc/clickhouse-server/config.xml
COPY database/configs/users.xml /etc/clickhouse-server/users.xml

# Set proper permissions
RUN chown -R clickhouse:clickhouse /etc/clickhouse-server/ && \
    chmod 644 /etc/clickhouse-server/config.xml && \
    chmod 644 /etc/clickhouse-server/users.xml

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=30s --retries=3 \
    CMD wget --spider -q localhost:8123/ping || exit 1

EXPOSE 8123 9000

# Copy initialization SQL scripts
RUN mkdir -p /docker-entrypoint-initdb.d
COPY database/build/init-lq-schema.sql /docker-entrypoint-initdb.d/01-init-lq-schema.sql
COPY database/build/init-ob-schema.sql /docker-entrypoint-initdb.d/02-init-ob-schema.sql
COPY database/build/init-pt-schema.sql /docker-entrypoint-initdb.d/03-init-pt-schema.sql
COPY database/build/init-sn-schema.sql /docker-entrypoint-initdb.d/04-init-sn-schema.sql

# Copy custom entrypoint script
COPY database-entrypoint.sh /custom-entrypoint.sh
RUN chmod +x /custom-entrypoint.sh

# Use custom entrypoint that will handle initialization
ENTRYPOINT ["/custom-entrypoint.sh"]

