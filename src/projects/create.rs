use axum::{extract, Extension, Json, response::IntoResponse, http::StatusCode};
use axum_sessions::extractors::ReadableSession;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::SqlitePool;

use crate::users::load_user;

#[derive(Deserialize)]
pub struct CreateProjectDto {
    pub name: String,
}

#[derive(Serialize)]
pub struct Project {
    pub id: Option<i64>,
    pub name: String,
    pub user_id: i64,
}

#[derive(Serialize)]
pub struct CreateProjectResponse {
    project: Project
}

pub enum CreateProjectErr {
    InvalidProjectName
}

impl CreateProjectDto {
    fn validate(&self) -> Result<(), CreateProjectErr> {
        if self.name.len() < 3 {
            return Err(CreateProjectErr::InvalidProjectName);
        }
        Ok(())
    }
}

impl IntoResponse for CreateProjectErr {
    fn into_response(self) -> axum::response::Response {
        let (status, message, fields) = match self {
            CreateProjectErr::InvalidProjectName => (StatusCode::BAD_REQUEST, "Invalid project name, use at least 3 letters", vec!["name"])
        };
        let body = Json(json!({
            "error": message, "fields": fields
        }));
        (status, body).into_response()
    }
}

pub async fn new(
    session: ReadableSession,
    Extension(pool): Extension<SqlitePool>,
    extract::Json(data): Json<CreateProjectDto>,
) -> Result<Json<CreateProjectResponse>, CreateProjectErr> {
    data.validate()?;
    let user = load_user(session, &pool).await;
    let project = sqlx::query_as!(
        Project,
        "INSERT INTO project (name, user_id) VALUES (?, ?) RETURNING *",
        data.name,
        user.id
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    Ok(Json(CreateProjectResponse { project }))
}
