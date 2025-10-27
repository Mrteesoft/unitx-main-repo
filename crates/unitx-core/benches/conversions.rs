use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_decimal::Decimal;
use std::str::FromStr;
use unitx_core::currency::{convert_with_provider, CurrencyUnit};
use unitx_core::distance::{convert as dist_convert, DistanceUnit};
use unitx_core::providers::LiveExchangeProvider;
use unitx_core::temperature::{convert as temp_convert, TemperatureUnit};

fn temperature_benchmarks(c: &mut Criterion) {
    c.bench_function("temperature_c_to_f", |b| {
        b.iter(|| temp_convert(black_box(37.5), TemperatureUnit::C, TemperatureUnit::F))
    });

    c.bench_function("temperature_f_to_k", |b| {
        b.iter(|| temp_convert(black_box(98.6), TemperatureUnit::F, TemperatureUnit::K))
    });

    c.bench_function("temperature_k_to_c", |b| {
        b.iter(|| temp_convert(black_box(273.15), TemperatureUnit::K, TemperatureUnit::C))
    });
}

fn distance_benchmarks(c: &mut Criterion) {
    c.bench_function("distance_km_to_mi", |b| {
        b.iter(|| dist_convert(black_box(10.0), DistanceUnit::KM, DistanceUnit::MI))
    });

    c.bench_function("distance_m_to_km", |b| {
        b.iter(|| dist_convert(black_box(5000.0), DistanceUnit::M, DistanceUnit::KM))
    });

    c.bench_function("distance_mi_to_m", |b| {
        b.iter(|| dist_convert(black_box(1.0), DistanceUnit::MI, DistanceUnit::M))
    });
}

fn currency_benchmarks(c: &mut Criterion) {
    let provider = LiveExchangeProvider::new(None);
    // Warm cache so the benchmarked loop uses cached values.
    provider
        .get_rate(CurrencyUnit::USD, CurrencyUnit::EUR)
        .expect("fetch live USD->EUR rate");
    let amount = Decimal::from_str("100.00").unwrap();

    c.bench_function("currency_live_usd_to_eur", move |b| {
        let provider = provider.clone();
        b.iter(|| {
            convert_with_provider(
                black_box(amount),
                CurrencyUnit::USD,
                CurrencyUnit::EUR,
                &provider,
            )
            .unwrap()
        })
    });
}

fn validation_benchmarks(c: &mut Criterion) {
    use unitx_core::validation::*;

    c.bench_function("validate_temperature", |b| {
        b.iter(|| validate_temperature_value(black_box(37.5)))
    });

    c.bench_function("validate_distance", |b| {
        b.iter(|| validate_distance_value(black_box(100.0)))
    });

    c.bench_function("validate_currency", |b| {
        b.iter(|| validate_currency_value(black_box("100.00")))
    });
}

fn parsing_benchmarks(c: &mut Criterion) {
    c.bench_function("parse_temperature_unit", |b| {
        b.iter(|| TemperatureUnit::parse(black_box("C")))
    });

    c.bench_function("parse_distance_unit", |b| {
        b.iter(|| DistanceUnit::parse(black_box("KM")))
    });

    c.bench_function("parse_currency_unit", |b| {
        b.iter(|| CurrencyUnit::parse(black_box("USD")))
    });
}

criterion_group!(
    benches,
    temperature_benchmarks,
    distance_benchmarks,
    currency_benchmarks,
    validation_benchmarks,
    parsing_benchmarks
);
criterion_main!(benches);
