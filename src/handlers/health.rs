use axum::{http::StatusCode, response::{IntoResponse, Json}};
use serde_json::{json, Value};

pub async fn health_check() -> impl IntoResponse {
    let body = Json(json!({
        "status": "ok",
        "message": "Service is running"
    }));

    (StatusCode::OK, body)
}
