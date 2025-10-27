# Changelog

All notable changes to this project are documented here. This project adheres to [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) and follows semantic versioning.

## [0.1.0] - 2025-10-27

### Added
- `unitx-core`: temperature, distance, and currency conversion primitives with validation helpers.
- Live currency provider powered by the European Central Bank daily feed (no API key required).
- Dedicated error types with `thiserror`, plus comprehensive unit tests and Criterion benchmarks.
- `unitx-api`: Axum-based HTTP service with CORS/logging middleware, JSON validation, and conversion endpoints for temperature, distance, and currency.
- Example binaries, documentation, and Dockerfile for local builds/deployment.
- Benchmarks, scripts, and Makefile targets covering build, test, bench, docker, and CI workflows.

### Docs & Tooling
- README quick start, examples, and publishing guidance.
- CONTRIBUTING, CODE_OF_CONDUCT, and community call for contributions.
- Continuous benchmarking guide (`BENCHMARKS.md`) and release checklist embedded in README.

