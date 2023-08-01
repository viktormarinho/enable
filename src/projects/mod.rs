use axum::{routing::get, Router, middleware};
use crate::users;

pub mod all;
pub mod create;

pub fn projects() -> Router {
    Router::new()
        .route("/", get(all::all).post(create::new))
        .layer(middleware::from_fn(users::require_auth))
}
