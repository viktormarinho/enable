use axum::{Router, Json, extract, routing::post, response::IntoResponse, http::StatusCode};
use sea_orm::sea_query::tests_cfg::json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct LoginDto {
    email: String,
    password: String
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

async fn login(extract::Json(data): Json<LoginDto>) -> impl IntoResponse {
    (StatusCode::OK, Json(LoginResponse { token: "zimbas".to_string() }))
}

#[derive(Deserialize)]
struct RegisterDto {
    email: String,
    password: String,
    passwordConfirm: String
}

#[derive(Serialize)]
struct RegisterResponse {
    token: String,
}

enum RegisterError {
    PasswordsDontMatch,
    EmailAlreadyTaken,    
}

impl IntoResponse for RegisterError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            RegisterError::EmailAlreadyTaken => (StatusCode::BAD_REQUEST, "Email already taken"),
            RegisterError::PasswordsDontMatch => (StatusCode::BAD_REQUEST, "Passwords do not match")
        };
        let body = Json(json!({
            "error": error_message
        }));
        (status, body).into_response()
    }
}

async fn register(extract::Json(data): Json<RegisterDto>) -> Result<(StatusCode, Json<RegisterResponse>), RegisterError> {
    if data.password != data.passwordConfirm {
        return Err(RegisterError::PasswordsDontMatch);
    }

    Ok((StatusCode::CREATED, Json(RegisterResponse { token: "zimbas".to_string() })))
}

pub fn auth() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}