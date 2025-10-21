use axum::{routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use rust_decimal::Decimal;
use std::str::FromStr;

// ------------ shared types ------------
#[derive(Serialize)]
struct Health { status: &'static str, version: &'static str }

#[derive(Deserialize)]
struct ConvertRequest {
    value: f64,
    from: String,
    to: String,
}

#[derive(Deserialize)]
struct CurrencyRequest {
    value: String,
    from: String,
    to: String,
    provider: Option<String>,
}

#[derive(Serialize)]
struct ConvertResponse {
    category: &'static str,
    from: String,
    to: String,
    input: f64,
    output: f64,
}

#[derive(Serialize)]
struct CurrencyResponse {
    category: &'static str,
    from: String,
    to: String,
    input: String,
    output: String,
}

#[tokio::main]
async fn main() {
    // logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "unitx_api=info,axum=warn".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // routes
    let app = Router::new()
        .route("/healthz", get(health))
        .route("/convert/temperature", post(convert_temperature))
        .route("/convert/distance", post(convert_distance))
        .route("/convert/currency", post(convert_currency));

    // server
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> Json<Health> {
    Json(Health { status: "ok", version: unitx_core::version() })
}

async fn convert_temperature(Json(req): Json<ConvertRequest>) -> Json<ConvertResponse> {
    use unitx_core::temperature::{convert, TemperatureUnit};
    let from_unit = TemperatureUnit::parse(&req.from).unwrap_or(TemperatureUnit::C);
    let to_unit   = TemperatureUnit::parse(&req.to).unwrap_or(TemperatureUnit::C);
    let output    = convert(req.value, from_unit, to_unit);

    Json(ConvertResponse {
        category: "temperature",
        from: req.from,
        to: req.to,
        input: req.value,
        output,
    })
}

async fn convert_distance(Json(req): Json<ConvertRequest>) -> Json<ConvertResponse> {
    use unitx_core::distance::{convert, DistanceUnit};
    let from_unit = DistanceUnit::parse(&req.from).unwrap_or(DistanceUnit::M);
    let to_unit   = DistanceUnit::parse(&req.to).unwrap_or(DistanceUnit::M);
    let output    = convert(req.value, from_unit, to_unit);

    Json(ConvertResponse {
        category: "distance",
        from: req.from,
        to: req.to,
        input: req.value,
        output,
    })
}

async fn convert_currency(Json(req): Json<CurrencyRequest>) -> Json<CurrencyResponse> {
    use unitx_core::currency::{convert_with_provider, CurrencyUnit};
    use unitx_core::providers::{MockProvider, FixedRateProvider};
    
    let value = Decimal::from_str(&req.value).unwrap_or(Decimal::ZERO);
    let from_unit = CurrencyUnit::parse(&req.from).unwrap_or(CurrencyUnit::USD);
    let to_unit   = CurrencyUnit::parse(&req.to).unwrap_or(CurrencyUnit::USD);
    
    let output = match req.provider.as_deref() {
        Some("fixed") => {
            let provider = FixedRateProvider::new();
            convert_with_provider(value, from_unit, to_unit, &provider)
                .unwrap_or(Decimal::ZERO)
        },
        _ => {
            let provider = MockProvider;
            convert_with_provider(value, from_unit, to_unit, &provider)
                .unwrap_or(Decimal::ZERO)
        }
    };

    Json(CurrencyResponse {
        category: "currency",
        from: req.from,
        to: req.to,
        input: req.value,
        output: output.to_string(),
    })
}