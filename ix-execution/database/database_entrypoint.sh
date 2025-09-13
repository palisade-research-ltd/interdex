#!/bin/bash
set -e

echo "=== ClickHouse Database Initialization Entrypoint ==="

# Remove any default password files that might override our configuration
rm -f /etc/clickhouse-server/users.d/default-password.xml
rm -f /etc/clickhouse-server/users.d/default-user.xml

# Ensure proper ownership
chown -R clickhouse:clickhouse /var/lib/clickhouse
chown -R clickhouse:clickhouse /var/log/clickhouse-server
chown -R clickhouse:clickhouse /etc/clickhouse-server

# Function to wait for ClickHouse to be ready
wait_for_clickhouse() {
    echo "Waiting for ClickHouse to start..."
    for i in {1..15}; do
        if clickhouse-client --query "SELECT 1" >/dev/null 2>&1; then
            echo "ClickHouse is ready!"
            return 0
        fi
        echo "Waiting for ClickHouse... attempt $i/30"
        sleep 5
    done
    echo "ERROR: ClickHouse failed to start within 60 seconds"
    return 1
}

# Function to execute SQL files
execute_init_scripts() {
    echo "Executing initialization scripts..."
    
    if [ -d "/docker-entrypoint-initdb.d" ]; then
        for f in /docker-entrypoint-initdb.d/*.sql; do
            if [ -f "$f" ]; then
                echo "Executing $f..."
                clickhouse-client --multiquery < "$f"
                echo "Completed $f"
            fi
        done
    fi
    
    echo "All initialization scripts completed!"
}

# Start ClickHouse server in background
echo "Starting ClickHouse server..."
exec /entrypoint.sh "$@"

# Wait for ClickHouse to be ready
if wait_for_clickhouse; then
    # Execute initialization scripts
    execute_init_scripts
    
    echo "=== ClickHouse initialization completed successfully ==="
    echo "ClickHouse is ready to accept connections"
else
    echo "=== ClickHouse initialization FAILED ==="
    exit 1
fi

