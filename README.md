# unitx

Fast, transparent **unit conversion** in Rust — offline-ready and open-source.  
API + core library: temperature, distance, and (mocked) currency with precise math.

## Why
- **Performance:** native speed (Rust) for thousands of conversions/sec.
- **Offline/Edge:** ship a tiny binary; no rate limits, no keys.
- **Transparency:** documented formulas, constants, and rounding.
- **Safety:** compile-time guarantees; predictable behavior in bigger systems.

## Structure

```
unitx-main/
├── Cargo.toml           # workspace
└── crates/
    ├── unitx-core/      # pure conversion logic (no HTTP)
    └── unitx-api/       # Axum HTTP server using unitx-core
```

## Quick start (dev)
```bash
# from repo root
cargo run -p unitx-api
# visit http://localhost:8080/healthz
```

## Roadmap (10 days)

- **Day 1:** skeleton + health check
- **Day 2:** temperature module + property tests
- **Day 3:** distance module
- **Day 4:** currency (mock) with precise decimals
- **Day 5:** REST endpoints
- **Day 6:** validation & errors
- **Day 7:** benchmarks
- **Day 8:** docs & examples
- **Day 9:** publish crate & container
- **Day 10:** release + call for PRs

## Contributing

Read CONTRIBUTING.md and open a small PR (tests/docs welcome!).
License: MIT. Code of Conduct: Contributor Covenant.