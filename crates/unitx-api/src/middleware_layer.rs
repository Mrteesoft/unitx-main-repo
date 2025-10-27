use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use std::time::Instant;

#[derive(Serialize, Debug)]
struct RequestLogEntry {
    method: String,
    path: String,
    duration_ms: u64,
    status: u16,
}

pub async fn logging_middleware(request: Request, next: Next) -> Response {
    let start = Instant::now();
    let method = request.method().to_string();
    let path = request.uri().path().to_string();

    let response = next.run(request).await;
    let duration = start.elapsed();

    let log_entry = RequestLogEntry {
        method,
        path,
        duration_ms: duration.as_millis() as u64,
        status: response.status().as_u16(),
    };

    tracing::info!("request completed: {:?}", log_entry);
    response
}

pub async fn cors_middleware(request: Request, next: Next) -> Response {
    let response = next.run(request).await;

    let mut response = response.into_response();
    let headers = response.headers_mut();

    headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    headers.insert(
        "Access-Control-Allow-Methods",
        "GET, POST, OPTIONS".parse().unwrap(),
    );
    headers.insert(
        "Access-Control-Allow-Headers",
        "Content-Type".parse().unwrap(),
    );

    response
}
