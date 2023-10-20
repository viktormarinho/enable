use axum::{extract, Extension, Json, response::IntoResponse, http::StatusCode};
use axum_sessions::extractors::ReadableSession;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::SqlitePool;

use crate::{users::load_user, models::{Project, Environment}};

#[derive(Deserialize)]
pub struct CreateProjectDto {
    pub name: String,
}

#[derive(Serialize)]
pub struct CreateProjectResponse {
    project: Project
}

pub enum CreateProjectErr {
    InvalidProjectName,
    CouldNotInsertOnDatabase,
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
            CreateProjectErr::InvalidProjectName => (StatusCode::BAD_REQUEST, "Invalid project name, use at least 3 letters", vec!["name"]),
            CreateProjectErr::CouldNotInsertOnDatabase => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error: Could not create the project", vec![])
        };
        let body = Json(json!({
            "error": message, "fields": fields
        }));
        (status, body).into_response()
    }
}

async fn create_default_envs(pool: &SqlitePool, project_id: String) -> Result<(), CreateProjectErr> {
    let dev_env = Environment::new(pool, "dev", &project_id);
    let prod_env = Environment::new(pool, "prod", &project_id);

    let (dev_env, prod_env) = tokio::join!(dev_env, prod_env);

    if dev_env.is_err() || prod_env.is_err() {
        return Err(CreateProjectErr::CouldNotInsertOnDatabase);
    }

    Ok(())
}

pub async fn new(
    session: ReadableSession,
    Extension(pool): Extension<SqlitePool>,
    extract::Json(data): Json<CreateProjectDto>,
) -> Result<Json<CreateProjectResponse>, CreateProjectErr> {
    data.validate()?;
    let user = load_user(session, &pool).await;
    let project = Project::new(data.name, user.id)
        .save(&pool)
        .await
        .map_err(|_| CreateProjectErr::CouldNotInsertOnDatabase)?;

    create_default_envs(&pool, project.id.clone().unwrap()).await?;

    Ok(Json(CreateProjectResponse { project }))
}
