#!/bin/bash
set -e

echo "=== Data Collector Startup ==="

# Wait for ClickHouse database to be ready
echo "Waiting for ClickHouse database..."
for i in {1..60}; do
    if wget --spider -q "${CLICKHOUSE_URL}/ping" 2>/dev/null; then
        echo "ClickHouse database is ready!"
        break
    fi
    echo "Waiting for database... attempt $i/60"
    sleep 2
done

if ! wget --spider -q "${CLICKHOUSE_URL}/ping" 2>/dev/null; then
    echo "ERROR: ClickHouse database not available after 120 seconds"
    exit 1
fi

echo "Starting data collector..."
# Change to appuser and execute the binary with correct path
su appuser -c "/usr/local/bin/datacollector"
