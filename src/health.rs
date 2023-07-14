use axum::{Router, Json, response::IntoResponse, http::StatusCode, routing::get};

#[derive(serde::Serialize)]
struct StatusResponse {
    status: String
}

async fn status() -> impl IntoResponse {
    // perform a health check here
    let status = "All systems operational".to_string();

    (StatusCode::OK, Json(StatusResponse { status }))
}

pub fn health() -> Router {
    Router::new()
        .route("/status", get(status))
}