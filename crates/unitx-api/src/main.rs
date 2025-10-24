use axum::{
    routing::{get, post}, Json, Router, http::StatusCode, 
    response::{IntoResponse, Response}, middleware::from_fn,
    extract::DefaultBodyLimit
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use rust_decimal::Decimal;
use std::str::FromStr;

mod middleware_layer;
use middleware_layer::{logging_middleware, cors_middleware};

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
    code: u16,
    timestamp: String,
}

#[derive(Serialize)]
struct ValidationErrorResponse {
    error: String,
    message: String,
    code: u16,
    field: Option<String>,
    timestamp: String,
}

type ApiResult<T> = Result<Json<T>, ApiError>;

struct ApiError {
    status: StatusCode,
    message: String,
    field: Option<String>,
}

impl ApiError {
    fn new(status: StatusCode, message: String) -> Self {
        Self { status, message, field: None }
    }
    
    fn with_field(status: StatusCode, message: String, field: String) -> Self {
        Self { status, message, field: Some(field) }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let timestamp = chrono::Utc::now().to_rfc3339();
        
        if let Some(field) = self.field {
            let body = Json(ValidationErrorResponse {
                error: "validation_error".to_string(),
                message: self.message,
                code: self.status.as_u16(),
                field: Some(field),
                timestamp,
            });
            (self.status, body).into_response()
        } else {
            let body = Json(ErrorResponse {
                error: "conversion_error".to_string(),
                message: self.message,
                code: self.status.as_u16(),
                timestamp,
            });
            (self.status, body).into_response()
        }
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
        .route("/convert/currency", post(convert_currency))
        .layer(DefaultBodyLimit::max(1024)) // 1KB max request size
        .layer(from_fn(cors_middleware))
        .layer(from_fn(logging_middleware));

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
    use unitx_core::validation::{validate_temperature_value, validate_unit_string};
    
    // Validate units
    validate_unit_string(&req.from, "Temperature").map_err(|e| 
        ApiError::with_field(StatusCode::BAD_REQUEST, e.to_string(), "from".to_string())
    )?;
    validate_unit_string(&req.to, "Temperature").map_err(|e| 
        ApiError::with_field(StatusCode::BAD_REQUEST, e.to_string(), "to".to_string())
    )?;
    
    // Validate value
    validate_temperature_value(req.value).map_err(|e| 
        ApiError::with_field(StatusCode::BAD_REQUEST, e.to_string(), "value".to_string())
    )?;
    
    let from_unit = TemperatureUnit::parse(&req.from).ok_or_else(|| 
        ApiError::with_field(StatusCode::BAD_REQUEST, 
            format!("Unsupported temperature unit: {} (supported: C, F, K)", req.from),
            "from".to_string())
    )?;
    
    let to_unit = TemperatureUnit::parse(&req.to).ok_or_else(|| 
        ApiError::with_field(StatusCode::BAD_REQUEST, 
            format!("Unsupported temperature unit: {} (supported: C, F, K)", req.to),
            "to".to_string())
    )?;
    
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
    validate_distance_value(req.value).map_err(|e| 
        ApiError::with_field(StatusCode::BAD_REQUEST, e.to_string(), "value".to_string())
    )?;
    
    let from_unit = DistanceUnit::parse(&req.from).ok_or_else(|| 
        ApiError::with_field(StatusCode::BAD_REQUEST, 
            format!("Unsupported distance unit: {} (supported: M, KM, MI)", req.from),
            "from".to_string())
    )?;
    
    let to_unit = DistanceUnit::parse(&req.to).ok_or_else(|| 
        ApiError::with_field(StatusCode::BAD_REQUEST, 
            format!("Unsupported distance unit: {} (supported: M, KM, MI)", req.to),
            "to".to_string())
    )?;
    
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
    validate_currency_value(&req.value).map_err(|e| 
        ApiError::with_field(StatusCode::BAD_REQUEST, e.to_string(), "value".to_string())
    )?;
    
    let value = Decimal::from_str(&req.value).map_err(|_| 
        ApiError::with_field(StatusCode::BAD_REQUEST, 
            "Invalid currency amount format".to_string(), "value".to_string())
    )?;
    
    let from_unit = CurrencyUnit::parse(&req.from).ok_or_else(|| 
        ApiError::with_field(StatusCode::BAD_REQUEST, 
            format!("Unsupported currency unit: {} (supported: USD, EUR, GBP, JPY)", req.from),
            "from".to_string())
    )?;
    
    let to_unit = CurrencyUnit::parse(&req.to).ok_or_else(|| 
        ApiError::with_field(StatusCode::BAD_REQUEST, 
            format!("Unsupported currency unit: {} (supported: USD, EUR, GBP, JPY)", req.to),
            "to".to_string())
    )?;
    
    let output = match req.provider.as_deref() {
        Some("fixed") => {
            let provider = FixedRateProvider::new();
            convert_with_provider(value, from_unit, to_unit, &provider)
                .map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e))?
        },
        _ => {
            let provider = MockProvider;
            convert_with_provider(value, from_unit, to_unit, &provider)
                .map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e))?
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