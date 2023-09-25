use axum::{http::{Request, StatusCode}, Extension, response::{Response, IntoResponse}, middleware::Next, Json};
use axum_sessions::extractors::ReadableSession;
use serde::{Serialize, Deserialize};
use serde_json::json;
use sqlx::SqlitePool;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    password_hash: String
}

pub enum LoadUserError {
    SessionDoesNotExist,
    UserDoesNotExist,
}

impl IntoResponse for LoadUserError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            LoadUserError::SessionDoesNotExist => (StatusCode::UNAUTHORIZED, "Session does not exist"),
            LoadUserError::UserDoesNotExist => (StatusCode::UNAUTHORIZED, "User does not exist"),
        };
        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}


// Tries to load the current user (works for routes that do not contain the users::require_auth middleware)
// Returns a error with IntoResponse implemented for easy error bubbling
pub async fn try_load_user(session: ReadableSession, pool: &SqlitePool) -> Result<User, LoadUserError> {
    let user_id = match session.get::<String>("user_id") {
        None => return Err(LoadUserError::SessionDoesNotExist),
        Some(usr) => usr,
    };

    let result = sqlx::query_as!(User, "SELECT * FROM user WHERE id = ?", user_id)
        .fetch_optional(pool)
        .await;

    match result {
        Ok(usr) => match usr {
            Some(usr) => Ok(usr),
            None => Err(LoadUserError::UserDoesNotExist)
        },
        Err(_) => Err(LoadUserError::UserDoesNotExist)
    }
}

/// ONLY USE THIS FUNCTION IN ROUTES THAT CONTAIN THE users::require_auth MIDDLEWARE
/// This function assumes that the current route uses this middleware and then utilize .unwrap
/// for convenience. If you want to error check use the users::try_load_user function
pub async fn load_user(session: ReadableSession, pool: &SqlitePool) -> User {
    let user_id = session.get::<String>("user_id").unwrap();

    sqlx::query_as!(User, "SELECT * FROM user WHERE id = ?", user_id)
        .fetch_optional(pool)
        .await
        .unwrap()
        .unwrap()
}

pub async fn require_auth<B>(
    Extension(pool): Extension<SqlitePool>,
    session: ReadableSession,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let user_id = session.get::<String>("user_id");

    if user_id.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let user = sqlx::query!("SELECT * FROM user WHERE id = ?", user_id)
        .fetch_optional(&pool)
        .await
        .unwrap();

    if user.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let response = next.run(request).await;
    Ok(response)
}