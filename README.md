# unitx

Fast, transparent **unit conversion** in Rust — offline-ready and open-source.  
API + core library: temperature, distance, and live currency conversions with precise math.

## Why
- **Performance:** native speed (Rust) for thousands of conversions/sec.
- **Live currency:** real FX data fetched on demand (European Central Bank daily feed by default).
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

## Examples

### Library (`unitx-core`)
```bash
cargo run -p unitx-core --example live_currency
```

The example fetches live USD → EUR rates using the public European Central Bank feed.

### API (`unitx-api`)
```bash
curl -X POST http://localhost:8080/convert/currency \
  -H "Content-Type: application/json" \
  -d '{"value":"100.00","from":"USD","to":"EUR"}'
```

For temperature or distance conversions, swap the path to `/convert/temperature` or `/convert/distance` and send numeric `value`.

## Live currency configuration

- **Default backend:** [European Central Bank daily reference rates](https://www.ecb.europa.eu/stats/policy_and_exchange_rates/euro_reference_exchange_rates/html/index.en.html) (no keys, updated every business day).
- **Source:** default feed is the European Central Bank daily XML (no API keys required).
- **Custom endpoint:** instantiate `LiveExchangeProvider::with_base_url()` for self-hosted mirrors or staging proxies.
- **Threading:** currency conversion runs in a blocking task inside the API so hot paths stay responsive.
- **Tests & benches:** the live currency test is ignored by default; run `cargo test -- --ignored` (with network access) to exercise it. Currency benchmarks warm the live provider on their first iteration.

## Publishing (Day 9)

### Crates.io
```bash
cargo publish -p unitx-core
cargo publish -p unitx-api
```

Ensure you have refreshed the version numbers beforehand and that `cargo test` and `cargo package --list` pass locally.

### Container image
```bash
make docker-build
make docker-run
```

The container exposes port `8080` and has no external configuration requirements for currency rates.

## Release (Day 10)

- Review and update `CHANGELOG.md`.
- Run `make release` to execute fmt/clippy/tests, package crates, and build the Docker image.
- Tag the repository, e.g. `git tag v0.1.0 && git push origin v0.1.0`.
- Announce the release with a short summary (link to README quick start and benchmarks).
- Open the floor for community contributions (see below).

## Roadmap (10 days)

- **Day 1:** skeleton + health check
- **Day 2:** temperature module + property tests
- **Day 3:** distance module
- **Day 4:** currency (live) with precise decimals
- **Day 5:** REST endpoints
- **Day 6:** validation & errors
- **Day 7:** benchmarks
- **Day 8:** docs & examples
- **Day 9:** publish crate & container
- **Day 10:** release + call for PRs

## Contributing

We’d love help expanding the unit catalog, improving validation, and hardening the HTTP API.  
- Check `CHANGELOG.md` for the latest release status.
- Look at open issues marked `help wanted` or propose new conversions/providers.
- Read `CONTRIBUTING.md`, then open a small PR (tests/docs welcome!).  

License: MIT. Code of Conduct: Contributor Covenant.
