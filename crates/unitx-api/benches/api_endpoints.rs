use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_decimal::Decimal;
use serde_json::json;
use std::str::FromStr;
use unitx_core::currency::{convert_with_provider, CurrencyUnit};
use unitx_core::distance::{convert as dist_convert, DistanceUnit};
use unitx_core::providers::LiveExchangeProvider;
use unitx_core::temperature::{convert as temp_convert, TemperatureUnit};

fn json_parsing_benchmarks(c: &mut Criterion) {
    c.bench_function("json_parse_temperature_request", |b| {
        let json_str = json!({
            "value": 37.5,
            "from": "C",
            "to": "F"
        })
        .to_string();

        b.iter(|| {
            let _: serde_json::Value = serde_json::from_str(black_box(&json_str)).unwrap();
        });
    });
}

fn full_conversion_pipeline_benchmarks(c: &mut Criterion) {
    c.bench_function("full_temperature_pipeline", |b| {
        b.iter(|| {
            let value = black_box(37.5);
            let from = TemperatureUnit::parse(black_box("C")).unwrap();
            let to = TemperatureUnit::parse(black_box("F")).unwrap();
            let result = temp_convert(value, from, to);
            black_box(result);
        });
    });

    c.bench_function("full_currency_pipeline", |b| {
        let provider = LiveExchangeProvider::new(None);
        provider
            .get_rate(CurrencyUnit::USD, CurrencyUnit::EUR)
            .expect("fetch live USD->EUR rate");
        b.iter(|| {
            let value = Decimal::from_str(black_box("100.00")).unwrap();
            let from = CurrencyUnit::parse(black_box("USD")).unwrap();
            let to = CurrencyUnit::parse(black_box("EUR")).unwrap();
            let result = convert_with_provider(value, from, to, &provider).unwrap();
            black_box(result);
        });
    });
}

criterion_group!(
    benches,
    json_parsing_benchmarks,
    full_conversion_pipeline_benchmarks
);
criterion_main!(benches);
