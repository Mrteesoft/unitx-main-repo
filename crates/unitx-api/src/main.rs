use axum::{routing::{get, post}, Json, Router, http::StatusCode, response::{IntoResponse, Response}};
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

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

type ApiResult<T> = Result<Json<T>, ApiError>;

struct ApiError {
    status: StatusCode,
    message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = Json(ErrorResponse {
            error: "conversion_error".to_string(),
            message: self.message,
        });
        (self.status, body).into_response()
    }
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

async fn convert_temperature(Json(req): Json<ConvertRequest>) -> ApiResult<ConvertResponse> {
    use unitx_core::temperature::{convert, TemperatureUnit};
    use unitx_core::validation::validate_temperature_value;
    
    // Validate input
    validate_temperature_value(req.value).map_err(|e| ApiError {
        status: StatusCode::BAD_REQUEST,
        message: e.to_string(),
    })?;
    
    let from_unit = TemperatureUnit::parse(&req.from).ok_or_else(|| ApiError {
        status: StatusCode::BAD_REQUEST,
        message: format!("Invalid temperature unit: {}", req.from),
    })?;
    
    let to_unit = TemperatureUnit::parse(&req.to).ok_or_else(|| ApiError {
        status: StatusCode::BAD_REQUEST,
        message: format!("Invalid temperature unit: {}", req.to),
    })?;
    
    let output = convert(req.value, from_unit, to_unit);

    Ok(Json(ConvertResponse {
        category: "temperature",
        from: req.from,
        to: req.to,
        input: req.value,
        output,
    }))
}

async fn convert_distance(Json(req): Json<ConvertRequest>) -> ApiResult<ConvertResponse> {
    use unitx_core::distance::{convert, DistanceUnit};
    use unitx_core::validation::validate_distance_value;
    
    // Validate input
    validate_distance_value(req.value).map_err(|e| ApiError {
        status: StatusCode::BAD_REQUEST,
        message: e.to_string(),
    })?;
    
    let from_unit = DistanceUnit::parse(&req.from).ok_or_else(|| ApiError {
        status: StatusCode::BAD_REQUEST,
        message: format!("Invalid distance unit: {}", req.from),
    })?;
    
    let to_unit = DistanceUnit::parse(&req.to).ok_or_else(|| ApiError {
        status: StatusCode::BAD_REQUEST,
        message: format!("Invalid distance unit: {}", req.to),
    })?;
    
    let output = convert(req.value, from_unit, to_unit);

    Ok(Json(ConvertResponse {
        category: "distance",
        from: req.from,
        to: req.to,
        input: req.value,
        output,
    }))
}

async fn convert_currency(Json(req): Json<CurrencyRequest>) -> ApiResult<CurrencyResponse> {
    use unitx_core::currency::{convert_with_provider, CurrencyUnit};
    use unitx_core::providers::{MockProvider, FixedRateProvider};
    use unitx_core::validation::validate_currency_value;
    
    // Validate input
    validate_currency_value(&req.value).map_err(|e| ApiError {
        status: StatusCode::BAD_REQUEST,
        message: e.to_string(),
    })?;
    
    let value = Decimal::from_str(&req.value).map_err(|_| ApiError {
        status: StatusCode::BAD_REQUEST,
        message: "Invalid currency amount format".to_string(),
    })?;
    
    let from_unit = CurrencyUnit::parse(&req.from).ok_or_else(|| ApiError {
        status: StatusCode::BAD_REQUEST,
        message: format!("Invalid currency unit: {}", req.from),
    })?;
    
    let to_unit = CurrencyUnit::parse(&req.to).ok_or_else(|| ApiError {
        status: StatusCode::BAD_REQUEST,
        message: format!("Invalid currency unit: {}", req.to),
    })?;
    
    let output = match req.provider.as_deref() {
        Some("fixed") => {
            let provider = FixedRateProvider::new();
            convert_with_provider(value, from_unit, to_unit, &provider)
                .map_err(|e| ApiError {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    message: e,
                })?
        },
        _ => {
            let provider = MockProvider;
            convert_with_provider(value, from_unit, to_unit, &provider)
                .map_err(|e| ApiError {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    message: e,
                })?
        }
    };

    Ok(Json(CurrencyResponse {
        category: "currency",
        from: req.from,
        to: req.to,
        input: req.value,
        output: output.to_string(),
    }))
}