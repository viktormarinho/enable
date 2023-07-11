use std::net::SocketAddr;

use axum::{routing::get, Router, Json};
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(serde::Serialize)]
struct Message {
    text: String,
}

async fn hello() -> Json<Message> {
    Json(Message {
        text: "Hello, World!".to_string(),
    })
}

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
        .route("/hello", get(hello));

    let app = Router::new()
        .nest_service("/", ServeDir::new("static"))
        .nest_service("/api", api)
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let port = 3000;

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
