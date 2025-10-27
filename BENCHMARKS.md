# unitx Benchmarks

Performance benchmarks for unitx conversion functions and API endpoints.

## Running Benchmarks

```bash
# Run all benchmarks
./scripts/benchmark.sh

# Run core library benchmarks only
cd crates/unitx-core && cargo bench

# Run API benchmarks only  
cd crates/unitx-api && cargo bench
```

## Benchmark Categories

### Core Library (`unitx-core`)
- **Temperature conversions**: C↔F↔K with different input values
- **Distance conversions**: M↔KM↔MI with various distances
- **Currency conversions**: USD↔EUR↔GBP↔JPY hitting the live provider (rates cached after the first fetch)
- **Validation functions**: Input validation for all conversion types
- **Unit parsing**: String to enum parsing performance

### API Endpoints (`unitx-api`)
- **POST /convert/temperature**: Full HTTP request/response cycle
- **POST /convert/distance**: Including JSON parsing and validation
- **POST /convert/currency**: End-to-end API performance (live FX rates, cached per benchmark)

## Expected Performance

Target performance goals:
- Core conversions: < 100ns per conversion
- Validation: < 50ns per validation
- API endpoints: < 1ms per request (excluding network)

## Viewing Results

Benchmark results are saved to `target/criterion/` with detailed HTML reports.
Open `target/criterion/report/index.html` in a browser for interactive charts and statistics.

## Continuous Benchmarking

Run benchmarks before major releases to ensure performance regressions are caught early. Currency benchmarks require outbound network access on the first iteration to warm the cache.
