use axum::{
    Json,
    http::{StatusCode, Uri},
    response::IntoResponse,
};
use serde_json::json;
use tracing::warn;

pub async fn not_found_handler(uri: Uri) -> impl IntoResponse {
    warn!("404 Not Found: {}", uri.path());
    let body = json!({
        "status": 404,
        "message": format!("No route found for '{}'", uri.path()),
    });

    (StatusCode::NOT_FOUND, Json(body))
}
