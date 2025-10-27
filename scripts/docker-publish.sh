#!/bin/bash
set -e

echo "Building and publishing unitx Docker container..."
echo "==============================================="

# Build container
echo "Building container..."
docker build -t unitx:latest .
docker build -t unitx:0.1.0 .

# Test container
echo "Testing container..."
docker run -d --name unitx-test -p 8080:8080 unitx:latest
sleep 3
curl -f http://localhost:8080/healthz || (echo "Health check failed" && exit 1)
docker stop unitx-test
docker rm unitx-test

echo "Container test passed!"

# Tag for registries
echo "Tagging for registries..."
docker tag unitx:latest mrteesoft/unitx:latest
docker tag unitx:0.1.0 mrteesoft/unitx:0.1.0
docker tag unitx:latest ghcr.io/mrteesoft/unitx:latest
docker tag unitx:0.1.0 ghcr.io/mrteesoft/unitx:0.1.0

echo ""
echo "Ready to publish containers!"
echo "Docker Hub:"
echo "  docker push mrteesoft/unitx:latest"
echo "  docker push mrteesoft/unitx:0.1.0"
echo ""
echo "GitHub Container Registry:"
echo "  docker push ghcr.io/mrteesoft/unitx:latest"
echo "  docker push ghcr.io/mrteesoft/unitx:0.1.0"