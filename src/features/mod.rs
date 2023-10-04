use axum::{Router, routing::{post, get}, middleware};

use crate::users::require_auth;

pub mod create;
pub mod all;
pub mod toggle;

pub fn features() -> Router {
    Router::new()
        .route("/:env_id", get(all::all))
        .route("/", post(create::new))
        .route("/toggle", post(toggle::toggle))
        .layer(middleware::from_fn(require_auth))
}