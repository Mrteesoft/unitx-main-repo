#!/bin/bash

echo "Running unitx benchmarks..."
echo "=========================="

echo "Core library benchmarks:"
cd crates/unitx-core
cargo bench --bench conversions

echo ""
echo "API endpoint benchmarks:"
cd ../unitx-api
cargo bench --bench api_endpoints

echo ""
echo "Benchmark results saved to target/criterion/"
echo "Open target/criterion/report/index.html to view detailed results"