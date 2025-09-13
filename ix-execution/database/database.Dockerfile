FROM clickhouse/clickhouse-server:latest

# Remove default password file that may override configuration
RUN rm -f /etc/clickhouse-server/users.d/default-password.xml

# Create required directories with proper permissions
RUN echo "create /var/lib/clickhouse folders"
RUN mkdir -p /var/lib/clickhouse/format_schemas && \
    mkdir -p /var/lib/clickhouse/access && \
    mkdir -p /var/lib/clickhouse/user_files && \
    mkdir -p /var/lib/clickhouse/tmp && \
    mkdir -p /var/log/clickhouse-server && \
    chown -R clickhouse:clickhouse /var/lib/clickhouse && \
    chown -R clickhouse:clickhouse /var/log/clickhouse-server

# Copy initialization SQL scripts to the init directory
RUN echo "Copy initialization queries.."
RUN mkdir -p /docker-entrypoint-initdb.d
COPY clickhouse/init-lq-schema.sql /docker-entrypoint-initdb.d/init-lq-schema.sql
COPY clickhouse/init-ob-schema.sql /docker-entrypoint-initdb.d/init-ob-schema.sql
COPY clickhouse/init-pt-schema.sql /docker-entrypoint-initdb.d/init-pt-schema.sql
COPY clickhouse/init-sn-schema.sql /docker-entrypoint-initdb.d/init-sn-schema.sql

# Set proper permissions for init directory
RUN chown -R clickhouse:clickhouse /docker-entrypoint-initdb.d/ && \
    chmod 644 /docker-entrypoint-initdb.d/*.sql

# Copy and set up custom entrypoint script
COPY database_entrypoint.sh /custom-entrypoint.sh
RUN chmod +x /custom-entrypoint.sh && \
    chown clickhouse:clickhouse /custom-entrypoint.sh

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=30s --retries=3 \
    CMD wget --spider -q localhost:8123/ping || exit 1

# Expose standard ClickHouse ports
EXPOSE 8123 9000 9009

# Use custom entrypoint
ENTRYPOINT ["/custom-entrypoint.sh"]
