use std::net::SocketAddr;

use axum::Router;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "enable=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let api = Router::new()
        .nest_service("/auth", enable::auth::auth())
        .nest_service("/health", enable::health::health());

    let app = Router::new()
        .nest_service("/", ServeDir::new("static"))
        .nest_service("/api", api)
        .layer(tower_http::trace::TraceLayer::new_for_http());

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
