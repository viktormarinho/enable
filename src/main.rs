use std::net::SocketAddr;

use argon2::password_hash::SaltString;
use axum::{
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::get,
    Extension, Router,
};
use axum_sessions::{async_session::MemoryStore, SessionLayer};
use rand::rngs::OsRng;
use rand::Rng;
use rust_embed::RustEmbed;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "enable=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let salt = SaltString::generate(&mut OsRng);

    let pool = enable::db::get_pool().await;

    let store = MemoryStore::new();
    let secret = rand::thread_rng().gen::<[u8; 128]>();
    let session_layer = SessionLayer::new(store, &secret);

    let api = Router::new()
        .nest_service("/auth", enable::auth::auth())
        .nest_service("/credentials", enable::external::credentials::credentials())
        .nest_service("/feature", enable::external::features::features())
        .nest_service("/features", enable::features::features())
        .nest_service("/projects", enable::projects::projects())
        .nest_service("/health", enable::health::health())
        .nest_service("/environment", enable::environment::environment())
        .layer(session_layer);

    let app = Router::new()
        .route("/", get(static_handler))
        .route("/*file", get(static_handler))
        .nest_service("/api", api)
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(Extension(pool))
        .layer(Extension(salt));

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

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();

    if path == "/" || path.len() == 0 {
        path = "index.html".to_string();
    }

    StaticFile(path)
}

#[derive(RustEmbed)]
#[folder = "static/"]
struct Asset;

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();

        match Asset::get(path.as_str()) {
            Some(content) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
            }
            None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
        }
    }
}
