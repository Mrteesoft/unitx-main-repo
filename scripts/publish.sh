#!/bin/bash
set -e

echo "Publishing unitx to crates.io..."
echo "================================"

# Pre-flight checks
echo "Running pre-flight checks..."
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --all -- --check

echo "Checking package contents..."
cargo package --list -p unitx-core
cargo package --list -p unitx-api

echo ""
echo "Ready to publish!"
echo "1. Ensure you're logged in: cargo login YOUR_API_TOKEN"
echo "2. Publish core first: cargo publish -p unitx-core"
echo "3. Wait 1-2 minutes for availability"
echo "4. Publish API: cargo publish -p unitx-api"
echo ""
echo "After publishing:"
echo "- Create GitHub release with tag v0.1.0"
echo "- Build and push Docker container"
echo "- Update documentation with installation instructions"