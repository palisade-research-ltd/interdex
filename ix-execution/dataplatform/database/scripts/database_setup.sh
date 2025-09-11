#!/bin/bash
set -e

echo "Starting ClickHouse with custom initialization..."

# Remove any existing default password files
rm -f /etc/clickhouse-server/users.d/default-password.xml

# Start ClickHouse server in background and capture PID
echo "Starting ClickHouse server for initialization..."
/usr/bin/clickhouse-server --config-file=/etc/clickhouse-server/config.xml --daemon --pidfile=/var/run/clickhouse-server/clickhouse-server.pid

# Wait for ClickHouse to be ready with longer timeout and more robust checking
echo "Waiting for ClickHouse to start..."
for i in {1..60}; do
    # Check if process is running first
    if pgrep clickhouse-server >/dev/null; then
        # Then check if it's responding to HTTP requests
        if curl -s --max-time 2 "http://localhost:8123/ping" >/dev/null 2>&1; then
            echo "ClickHouse is ready!"
            break
        fi
    fi
    echo "Waiting for ClickHouse... ($i/60)"
    sleep 2
done

# Verify ClickHouse is actually ready
if ! curl -s --max-time 5 "http://localhost:8123/ping" >/dev/null 2>&1; then
    echo "ERROR: ClickHouse failed to start properly"
    # Show some debug info
    echo "Process status:"
    pgrep clickhouse-server || echo "No clickhouse-server process found"
    echo "Port status:"
    netstat -tlpn | grep -E "(8123|9000)" || echo "No ClickHouse ports listening"
    echo "Recent logs:"
    tail -n 20 /var/log/clickhouse-server/clickhouse-server.log
    exit 1
fi

# Execute initialization SQL scripts if they exist
if [ -d "/docker-entrypoint-initdb.d" ]; then
    echo "Running initialization scripts..."
    for f in /docker-entrypoint-initdb.d/*; do
        case "$f" in
            *.sql)
                echo "Executing $f..."
                # Use HTTP interface instead of native client for more reliability
                if ! clickhouse-client --host 127.0.0.1 --port 9000 --multiquery < "$f"; then
                    echo "WARNING: Failed to execute $f, trying HTTP interface..."
                    # Fallback to HTTP if native client fails
                    curl -X POST "http://localhost:8123/" --data-binary @"$f"
                fi
                ;;
            *.sh)
                echo "Running $f..."
                bash "$f"
                ;;
        esac
    done
    echo "Initialization scripts completed."
fi

# Stop the background server gracefully
echo "Stopping background ClickHouse server..."
if [ -f "/var/run/clickhouse-server/clickhouse-server.pid" ]; then
    PID=$(cat /var/run/clickhouse-server/clickhouse-server.pid)
    kill -TERM "$PID" 2>/dev/null || true
    # Wait for graceful shutdown
    for i in {1..10}; do
        if ! kill -0 "$PID" 2>/dev/null; then
            break
        fi
        sleep 1
    done
    # Force kill if still running
    kill -KILL "$PID" 2>/dev/null || true
else
    pkill clickhouse-server || true
fi

# Wait a moment for complete shutdown
sleep 3

echo "Starting ClickHouse server in foreground..."
# Start ClickHouse server in foreground
exec /usr/bin/clickhouse-server --config-file=/etc/clickhouse-server/config.xml
