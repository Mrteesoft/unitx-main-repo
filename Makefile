.PHONY: test bench clean build run docker-build docker-run release

# Default target
all: build test

# Build all packages
build:
	cargo build --workspace

# Run all tests
test:
	cargo test --workspace

# Run benchmarks
bench:
	./scripts/benchmark.sh

# Run core benchmarks only
bench-core:
	cd crates/unitx-core && cargo bench

# Run API benchmarks only
bench-api:
	cd crates/unitx-api && cargo bench

# Clean build artifacts
clean:
	cargo clean

# Run the API server
run:
	cargo run -p unitx-api

# Build Docker image for the API server
docker-build:
	docker build -t unitx-api:latest .

# Run the API server via Docker
docker-run:
	docker run --rm -p 8080:8080 unitx-api:latest

# Pre-release sanity check (fmt, clippy, tests, package manifests, docker)
release: ci
	cargo package -p unitx-core
	cargo package -p unitx-api
	$(MAKE) docker-build

# Check code formatting
fmt-check:
	cargo fmt --all -- --check

# Format code
fmt:
	cargo fmt --all

# Run clippy
clippy:
	cargo clippy --workspace --all-targets -- -D warnings

# Full CI check
ci: fmt-check clippy test

# Development setup
dev: build test bench

# Docker commands
docker-build:
	docker build -t unitx:latest .

docker-run:
	docker run -p 8080:8080 unitx:latest

docker-test:
	docker build -t unitx:test . && docker run --rm -p 8080:8080 -d --name unitx-test unitx:test && sleep 2 && curl -f http://localhost:8080/healthz && docker stop unitx-test

# Release workflow
release: ci docker-build
	@echo "Release ready - run 'make docker-test' to verify container"
