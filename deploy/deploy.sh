#!/bin/bash
# SecureBeam Deployment Script
# Run on the target server to deploy/update SecureBeam

set -e

DEPLOY_DIR="/opt/securebeam"
VERSION="${1:-latest}"

echo "=== SecureBeam Deployment ==="
echo "Version: $VERSION"
echo "Deploy Dir: $DEPLOY_DIR"

# Create deployment directory
sudo mkdir -p $DEPLOY_DIR
cd $DEPLOY_DIR

# Login to GitHub Container Registry (if credentials available)
if [ -n "$GITHUB_TOKEN" ]; then
    echo "$GITHUB_TOKEN" | docker login ghcr.io -u "$GITHUB_USER" --password-stdin
fi

# Pull latest images
echo "Pulling latest images..."
docker pull ghcr.io/tecburst/securebeam-frontend:$VERSION || echo "Frontend image not available, will build locally"
docker pull ghcr.io/tecburst/securebeam-mailbox:$VERSION || echo "Mailbox image not available, will build locally"
docker pull ghcr.io/tecburst/securebeam-relay:$VERSION || echo "Relay image not available, will build locally"

# Export version for docker-compose
export VERSION=$VERSION

# Stop existing containers
echo "Stopping existing containers..."
docker compose down --remove-orphans 2>/dev/null || true

# Start new containers
echo "Starting containers..."
docker compose up -d

# Cleanup old images
echo "Cleaning up old images..."
docker image prune -f

# Show status
echo ""
echo "=== Deployment Complete ==="
docker compose ps

echo ""
echo "Services:"
echo "  - Frontend: http://159.195.74.237"
echo "  - Mailbox Server: ws://159.195.74.237:3030"
echo "  - Relay Server: tcp://159.195.74.237:4001"
