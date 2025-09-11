# Alternative Simpler Database Entrypoint (Recommended)
#!/bin/bash
set -e

echo "Starting ClickHouse server..."

# Remove any existing default password files
rm -f /etc/clickhouse-server/users.d/default-password.xml

# Create PID directory
mkdir -p /var/run/clickhouse-server
chown clickhouse:clickhouse /var/run/clickhouse-server

# Execute initialization SQL scripts BEFORE starting the server
# This avoids the connection issues entirely
if [ -d "/docker-entrypoint-initdb.d" ]; then
    echo "Preprocessing initialization scripts..."
    
    # Start ClickHouse temporarily for initialization
    echo "Starting temporary ClickHouse for initialization..."
    su-exec clickhouse /usr/bin/clickhouse-server --config-file=/etc/clickhouse-server/config.xml --daemon --pidfile=/var/run/clickhouse-server/clickhouse-server.pid &
    
    # Wait with better error handling
    echo "Waiting for temporary server..."
    for i in {1..30}; do
        if curl -s --connect-timeout 1 --max-time 2 "http://localhost:8123/ping" 2>/dev/null | grep -q "Ok"; then
            echo "Temporary server is ready!"
            break
        fi
        if [ $i -eq 30 ]; then
            echo "Failed to start temporary server for initialization"
            # Show debug info
            ps aux | grep clickhouse || true
            netstat -tlpn | grep -E "(8123|9000)" || true
            tail -n 10 /var/log/clickhouse-server/clickhouse-server.log 2>/dev/null || true
            exit 1
        fi
        echo "Waiting... ($i/30)"
        sleep 2
    done
    
    # Run initialization scripts
    for f in /docker-entrypoint-initdb.d/*.sql; do
        if [ -f "$f" ]; then
            echo "Executing $(basename $f)..."
            clickhouse-client --host localhost --multiquery < "$f" || {
                echo "ERROR: Failed to execute $f"
                exit 1
            }
        fi
    done
    
    # Stop temporary server
    echo "Stopping temporary server..."
    if [ -f "/var/run/clickhouse-server/clickhouse-server.pid" ]; then
        kill $(cat /var/run/clickhouse-server/clickhouse-server.pid) || true
        rm -f /var/run/clickhouse-server/clickhouse-server.pid
    fi
    
    # Wait for shutdown
    sleep 3
    
    echo "Initialization completed!"
fi

# Start the main server
echo "Starting ClickHouse server in foreground..."
exec su-exec clickhouse /usr/bin/clickhouse-server --config-file=/etc/clickhouse-server/config.xml
