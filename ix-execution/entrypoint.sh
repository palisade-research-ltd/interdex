#!/bin/bash
set -e

# Remove any default password files that might override our configuration
rm -f /etc/clickhouse-server/users.d/default-password.xml
rm -f /etc/clickhouse-server/users.d/default-user.xml

# Ensure proper ownership
chown -R clickhouse:clickhouse /var/lib/clickhouse
chown -R clickhouse:clickhouse /var/log/clickhouse-server
chown -R clickhouse:clickhouse /etc/clickhouse-server

# Start ClickHouse server
exec /entrypoint.sh "$@"
