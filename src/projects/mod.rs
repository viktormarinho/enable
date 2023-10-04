use axum::{routing::get, Router, middleware};
use crate::users;

pub mod all;
pub mod create;
pub mod meta;

pub fn projects() -> Router {
    Router::new()
        .route("/", get(all::all).post(create::new))
        .route("/meta/:project_id", get(meta::meta))
        .layer(middleware::from_fn(users::require_auth))
}
