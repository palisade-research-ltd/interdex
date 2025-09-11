#!/bin/bash

# Read environment variables from user_data (not hardcoded)
# S3_BUCKET_NAME=${S3_BUCKET_NAME:-"infradex-datalake-deployment-files"}
# PROJECT_ID=${PROJECT_ID:-"infradex"}
# AWS_REGION=${AWS_REGION:-"us-east-1"}

# Read environment variables set by user_data
export S3_BUCKET_NAME="infradex-datalake-deployment-files"
export PROJECT_ID="infradex"
export AWS_REGION="us-east-1"

# Continue logging to the same file started by user_data_base64 in the main.tf
exec >> /var/log/datalake-deployment.log 2>&1

echo "=== DATA COLLECTOR DEPLOYMENT ==="
echo "Starting data collector deployment at $(date)"
echo "Using S3 bucket: $S3_BUCKET_NAME"
echo "Project ID: $PROJECT_ID"
echo "AWS Region: $AWS_REGION"

# Create directory structure
echo "Creating directory structure..."
mkdir -p /opt/infradex/datacollector/{build,configs,scripts}

# Download files from S3
echo "Downloading files from S3 bucket: $S3_BUCKET_NAME"
aws s3 sync s3://$S3_BUCKET_NAME/datacollector/build/ /opt/infradex/datacollector/build/ || exit 1
aws s3 sync s3://$S3_BUCKET_NAME/datacollector/configs/ /opt/infradex/datacollector/configs/ || exit 1
aws s3 sync s3://$S3_BUCKET_NAME/datacollector/scripts/ /opt/infradex/datacollector/scripts/ || exit 1

echo "Files downloaded successfully. Contents:"
ls -la /opt/infradex/datacollector/build/
ls -la /opt/infradex/datacollector/configs/
ls -la /opt/infradex/datacollector/scripts/

# Detect architecture and copy correct binary
ARCH=$(uname -m)
echo "Detected architecture: $ARCH"

if [ "$ARCH" = "x86_64" ]; then
    BINARY_NAME="datacollector_x86_64"
elif [ "$ARCH" = "aarch64" ]; then
    BINARY_NAME="datacollector_arm_64"
else
    echo "Unsupported architecture: $ARCH"
    exit 1
fi

echo "Using binary: $BINARY_NAME"

# Set permissions
echo "Setting permissions..."
chown -R ubuntu:ubuntu /opt/infradex
chmod +x /opt/infradex/datacollector/build/$BINARY_NAME 2>/dev/null || true
chmod +x /opt/infradex/datacollector/scripts/*.sh 2>/dev/null || true

# Change to collector directory for proper build context
# cd /opt/infradex/datacollector

# Build Docker image from the correct location
echo "Building Collector Docker Image..."
if [ -f "/opt/infradex/datacollector/build/datacollector.Dockerfile" ]; then
  # Change to the correct directory BEFORE building
  cd /opt/infradex
  
  # Verify we're in the right place
  echo "Current directory: $(pwd)"
  echo "Contents of datacollector/build/:"
  ls -la datacollector/build/ || echo "datacollector/build/ not found"
  echo "Contents of datacollector/configs/:"
  ls -la datacollector/configs/ || echo "datacollector/configs/ not found"
  
  # Build with correct context and dockerfile path
  docker build --no-cache -f datacollector/build/datacollector.Dockerfile -t datacollector . || exit 1
  echo "Datacollector Docker image built successfully"
  
  # Wait for ClickHouse to be ready before starting collector
  echo "Waiting for ClickHouse to be ready..."
  for i in {1..30}; do
    if curl -s http://localhost:8123/ping | grep -q "Ok"; then
      echo "ClickHouse is ready!"
      break
    else
      echo "Waiting for ClickHouse... ($i/30)"
      sleep 10
    fi
  done

  # Create a Docker network for container communication
  #echo "Creating Docker network..."
  #docker network create infradex-network 2>/dev/null || echo "Network already exists"
  
  # Connect ClickHouse container to the network
  echo "Connecting ClickHouse to network..."
  docker network connect infradex-network database-clickhouse 2>/dev/null || echo "Already connected"
  
  # Run datacollector container with proper networking
  echo "Starting datacollector container..."

  docker run -d \
    --name datacollector \
    --network infradex-network \
    -e RUST_LOG=debug \
    -e CLICKHOUSE_URL=http://database-clickhouse:8123 \
    -e CONFIG_PATH=/app/config/datacollector_config.toml \
    -v /var/log:/app/logs \
    --restart=no \
    datacollector:latest || exit 1

  echo "DataCollector container started successfully"

  # Verify connectivity between containers
  echo "Testing connectivity..."
  sleep 10
  docker exec datacollector curl -s http://database-clickhouse:8123/ping || echo "Connectivity test failed"
  
  # Show container logs
  echo "DataCollector logs (last 20 lines):"
  docker logs datacollector --tail 20

  # Verify both containers are running
  echo "Container status:"
  docker ps
  
else
  echo "ERROR: datacollector.Dockerfile not found in build/"
  ls -la build/
  exit 1
fi

echo "datacollector deployment completed successfully at $(date)"

