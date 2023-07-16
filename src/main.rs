use std::{net::SocketAddr, sync::Arc, collections::HashMap};

use argon2::{password_hash::SaltString};
use axum::{Router, Extension};
use rand::rngs::OsRng;
use tokio::sync::Mutex;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "enable=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let salt = SaltString::generate(&mut OsRng);

    let sessions = Arc::new(Mutex::new(HashMap::<String, u64>::new()));

    let pool = SqlitePool::connect(&std::env::var("DATABASE_URL")
        .expect("DATABASE_URL env var not set")).await.unwrap();

    let api = Router::new()
        .nest_service("/auth", enable::auth::auth())
        .nest_service("/health", enable::health::health());

    let app = Router::new()
        .nest_service("/", ServeDir::new("static"))
        .nest_service("/api", api)
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(Extension(pool))
        .layer(Extension(salt))
        .layer(Extension(sessions));

    let port: u16 = std::env::var("ENABLE_PORT")
        .unwrap_or(String::from("3000"))
        .parse()
        .unwrap_or(3000);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
