use axum::{Router, routing::{post, get}};

pub mod create;
pub mod all;

pub fn features() -> Router {
    Router::new()
        .route("/:project_id", get(all::all))
        .route("/", post(create::new))
}