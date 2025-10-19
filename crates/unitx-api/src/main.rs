use axum::{extract::Query, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// ------------ shared types ------------
#[derive(Serialize)]
struct Health { status: &'static str, version: &'static str }

// ------------ temperature endpoint ------------
#[derive(Deserialize)]
struct TempQuery { value: f64, from: String, to: String }

#[derive(Serialize)]
struct ConvertResponse {
    category: &'static str,
    from: String,
    to: String,
    input: f64,
    output: f64,
}

// ------------ distance endpoint ------------
#[derive(Deserialize)]
struct DistQuery { value: f64, from: String, to: String }

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
        .route("/convert/temperature", get(convert_temperature))
        .route("/convert/distance", get(convert_distance));

    // server
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> Json<Health> {
    Json(Health { status: "ok", version: unitx_core::version() })
}

async fn convert_temperature(Query(q): Query<TempQuery>) -> Json<ConvertResponse> {
    use unitx_core::temperature::{convert, TemperatureUnit};
    let from = TemperatureUnit::parse(&q.from).unwrap_or(TemperatureUnit::C);
    let to   = TemperatureUnit::parse(&q.to).unwrap_or(TemperatureUnit::C);
    let out  = convert(q.value, from, to);

    Json(ConvertResponse {
        category: "temperature",
        from: q.from, to: q.to,
        input: q.value, output: out,
    })
}

async fn convert_distance(Query(q): Query<DistQuery>) -> Json<ConvertResponse> {
    use unitx_core::distance::{convert, DistanceUnit};
    let from = DistanceUnit::parse(&q.from).unwrap_or(DistanceUnit::M);
    let to   = DistanceUnit::parse(&q.to).unwrap_or(DistanceUnit::M);
    let out  = convert(q.value, from, to);

    Json(ConvertResponse {
        category: "distance",
        from: q.from, to: q.to,
        input: q.value, output: out,
    })
}