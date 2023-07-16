use argon2::password_hash::SaltString;
use axum::{Router, Json, extract, routing::{post, get}, response::IntoResponse, http::StatusCode, Extension};
use axum_sessions::{async_session::MemoryStore, SessionLayer, extractors::{WritableSession, ReadableSession}};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::SqlitePool;

#[derive(Deserialize)]
struct LoginDto {
    email: String,
    password: String
}

#[derive(Serialize)]
struct LoginResponse {
    msg: String,
}

fn generate_session_token() -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(128)
        .map(char::from)
        .collect()
}

enum LoginError {
    UserDoesNotExist,
    InvalidEmailOrPassword,
    ErrorCreatingSession
}

impl IntoResponse for LoginError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            LoginError::UserDoesNotExist => (StatusCode::BAD_REQUEST, "User does not exist"),
            LoginError::InvalidEmailOrPassword => (StatusCode::BAD_REQUEST, "Invalid email or password"),
            LoginError::ErrorCreatingSession => (StatusCode::INTERNAL_SERVER_ERROR, "Error creating session")
        };
        let body = Json(json!({
            "error": error_message
        }));
        (status, body).into_response()
    }
}

async fn login(Extension(pool): Extension<SqlitePool>, mut session: WritableSession, extract::Json(data): Json<LoginDto>) -> Result<(StatusCode, Json<LoginResponse>), LoginError> {
    let user = sqlx::query!("SELECT * FROM user WHERE email = ?", data.email)
        .fetch_optional(&pool)
        .await
        .unwrap();

    if user.is_none() {
        return Err(LoginError::UserDoesNotExist);
    }

    let user = user.unwrap();

    if !crate::hash::verify(user.password_hash, data.password) {
        return Err(LoginError::InvalidEmailOrPassword);
    }
    
    if session.insert("user_id", user.id).is_err() {
        return Err(LoginError::ErrorCreatingSession);
    }
    
    Ok((StatusCode::OK, Json(LoginResponse { msg: "Session started with success".to_string() })))
}

#[derive(Deserialize)]
struct RegisterDto {
    email: String,
    password: String,
    #[serde(rename = "passwordConfirm")]
    password_confirm: String
}

#[derive(Serialize)]
struct RegisterResponse {
    msg: String,
}

enum RegisterError {
    PasswordsDontMatch,
    EmailAlreadyTaken,
    ErrorCreatingSession,
}

impl IntoResponse for RegisterError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            RegisterError::EmailAlreadyTaken => (StatusCode::BAD_REQUEST, "Email already taken"),
            RegisterError::PasswordsDontMatch => (StatusCode::BAD_REQUEST, "Passwords do not match"),
            RegisterError::ErrorCreatingSession => (StatusCode::INTERNAL_SERVER_ERROR, "Error creating session")
        };
        let body = Json(json!({
            "error": error_message
        }));
        (status, body).into_response()
    }
}

async fn register(mut session: WritableSession, Extension(salt): Extension<SaltString>, Extension(pool): Extension<SqlitePool>, extract::Json(data): Json<RegisterDto>) -> Result<(StatusCode, Json<RegisterResponse>), RegisterError> {
    if data.password != data.password_confirm {
        return Err(RegisterError::PasswordsDontMatch);
    }

    let user = sqlx::query!("SELECT * FROM user WHERE email = ?", data.email)
        .fetch_optional(&pool)
        .await
        .unwrap();

    if user.is_some() {
        return Err(RegisterError::EmailAlreadyTaken);
    }

    let password_hash = crate::hash::hash(data.password, &salt).unwrap();

    let user = sqlx::query!(
        "INSERT INTO user (email, password_hash) VALUES (?, ?) RETURNING id",
        data.email,
        password_hash
    ).fetch_one(&pool).await.unwrap();

    if session.insert("user_id", user.id).is_err() {
        return Err(RegisterError::ErrorCreatingSession);
    }

    Ok((StatusCode::CREATED, Json(RegisterResponse { msg: "Session started with success".to_string() })))
}

#[derive(Serialize)]
struct MeResponse {
    email: String
}

enum MeError {
    SessionDoesNotExist,
    UserDoesNotExist
}

impl IntoResponse for MeError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            MeError::SessionDoesNotExist => (StatusCode::BAD_REQUEST, "Session does not exist"),
            MeError::UserDoesNotExist => (StatusCode::BAD_REQUEST, "User does not exist")
        };
        let body = Json(json!({
            "error": error_message
        }));
        (status, body).into_response()
    }
}

async fn me(session: ReadableSession, Extension(pool): Extension<SqlitePool>) -> Result<(StatusCode, Json<MeResponse>), MeError> {
    let user_id = session.get::<i64>("user_id");

    if user_id.is_none() {
        return Err(MeError::SessionDoesNotExist);
    }

    let user = sqlx::query!("SELECT * FROM user WHERE id = ?", user_id)
        .fetch_optional(&pool)
        .await
        .unwrap();

    if user.is_none() {
        return Err(MeError::UserDoesNotExist);
    }

    let user = user.unwrap();

    Ok((StatusCode::OK, Json(MeResponse { email: user.email })))
}

pub fn auth() -> Router {
    let store = MemoryStore::new();
    let secret = rand::thread_rng().gen::<[u8; 128]>();
    let session_layer = SessionLayer::new(store, &secret);

    Router::new()
        .route("/me", get(me))
        .route("/login", post(login))
        .route("/register", post(register))
        .layer(session_layer)
}