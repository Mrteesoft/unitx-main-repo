.PHONY: test bench clean build run

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