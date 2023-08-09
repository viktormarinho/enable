use axum::{middleware, routing::{post, delete, get}, Extension, Json, Router, extract::Path};
use rand::{distributions::Uniform, Rng};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::users;

#[derive(Serialize)]
pub struct TokenRemovalError(String);

#[derive(Serialize)]
pub struct DeleteTokenResponse {
    pub deleted_id: i64
}

async fn remove_external_token(pool: &SqlitePool, id: i64) -> Result<i64, TokenRemovalError> {
    let result = sqlx::query!("DELETE FROM credentials WHERE id = ?;", id)
        .execute(pool)
        .await;

    if result.is_err() {
        return Err(TokenRemovalError("Could not remove the token from the database".to_string()));
    }

    Ok(id)
}

pub async fn delete_external_token(Extension(pool): Extension<SqlitePool>, Path(id): Path<i64>) -> Result<Json<DeleteTokenResponse>, Json<TokenRemovalError>> {
    let result = remove_external_token(&pool, id).await;

    if result.is_err() {
        return Err(Json(result.unwrap_err()));
    }

    Ok(Json(DeleteTokenResponse { deleted_id: id }))
}

struct CreateToken {
    name: String,
    token: String,
}

#[derive(Serialize)]
struct CredentialCreationError(String);

async fn add_external_token(
    pool: &SqlitePool,
    data: CreateToken,
) -> Result<Credential, CredentialCreationError> {
    match sqlx::query_as!(
        Credential,
        "INSERT INTO credentials (name, token) VALUES (?, ?) RETURNING *;",
        data.name,
        data.token
    )
    .fetch_one(pool)
    .await
    {
        Ok(val) => Ok(val),
        Err(_) => Err(CredentialCreationError(
            "Could not save new token in the database".to_string(),
        )),
    }
}

fn gen_external_token() -> String {
    rand::thread_rng()
        .sample_iter(Uniform::new(char::from(32), char::from(126)))
        .take(128)
        .map(char::from)
        .collect()
}

#[derive(Deserialize)]
struct NewCredentialDto {
    name: String,
}

#[derive(Serialize)]
struct Credential {
    id: i64,
    name: String,
    token: String,
}

async fn new_token(
    Extension(pool): Extension<SqlitePool>,
    axum::extract::Json(data): Json<NewCredentialDto>,
) -> Result<Json<Credential>, Json<CredentialCreationError>> {
    let token = gen_external_token();
    let token = add_external_token(
        &pool,
        CreateToken {
            name: data.name,
            token,
        },
    )
    .await;

    match token {
        Ok(t) => Ok(Json(t)),
        Err(e) => Err(Json(e))
    }
}

#[derive(Serialize)]
pub struct ListCredentialsError(String);

async fn list_credentials(
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<Vec<Credential>>, Json<ListCredentialsError>> {
    let creds = sqlx::query_as!(
        Credential,
        r#"SELECT * FROM credentials;"#
    ).fetch_all(&pool)
    .await
    .unwrap();

    Ok(Json(creds))
}

pub fn credentials() -> Router {
    Router::new()
        .route("/", get(list_credentials))
        .route("/", post(new_token))
        .route("/:id", delete(delete_external_token))
        .layer(middleware::from_fn(users::require_auth))
}
