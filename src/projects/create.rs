use axum::{extract, http::StatusCode, response::IntoResponse, Extension, Json};
use axum_sessions::extractors::ReadableSession;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::SqlitePool;

use crate::{
    models::{Environment, Project},
    users::load_user,
};

#[derive(Deserialize)]
pub struct CreateProjectDto {
    pub name: String,
}

#[derive(Serialize)]
pub struct CreateProjectResponse {
    project: Project,
}

pub enum CreateProjectErr {
    NameTooShort,
    NameAlreadyTaken,
    CouldNotInsertOnDatabase,
}

impl CreateProjectDto {
    async fn validate(&self, pool: &SqlitePool) -> Result<(), CreateProjectErr> {
        if self.name.len() < 3 {
            return Err(CreateProjectErr::NameTooShort);
        }
        let project = Project::get_by_name(pool, self.name.clone())
            .await
            .map_err(|_| CreateProjectErr::CouldNotInsertOnDatabase)?;

        if project.is_some() {
            return Err(CreateProjectErr::NameAlreadyTaken);
        }

        Ok(())
    }
}

impl IntoResponse for CreateProjectErr {
    fn into_response(self) -> axum::response::Response {
        let (status, message, fields) = match self {
            CreateProjectErr::NameTooShort => (
                StatusCode::BAD_REQUEST,
                "Invalid project name, use at least 3 letters",
                vec!["name"],
            ),
            CreateProjectErr::NameAlreadyTaken => (
                StatusCode::BAD_REQUEST,
                "Project name has already been taken",
                vec!["name"],
            ),
            CreateProjectErr::CouldNotInsertOnDatabase => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal error: Could not create the project",
                vec![],
            ),
        };
        let body = Json(json!({
            "error": message, "fields": fields
        }));
        (status, body).into_response()
    }
}

async fn create_default_envs(
    pool: &SqlitePool,
    project_id: String,
) -> Result<(), CreateProjectErr> {
    let dev_env = Environment::new("dev", project_id.clone()).save(pool).await;
    let prod_env = Environment::new("prod", project_id).save(pool).await;

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
    data.validate(&pool).await?;
    let user = load_user(session, &pool).await;
    let project = Project::new(data.name, user.id)
        .save(&pool)
        .await
        .map_err(|_| CreateProjectErr::CouldNotInsertOnDatabase)?;

    create_default_envs(&pool, project.id.clone().unwrap()).await?;

    Ok(Json(CreateProjectResponse { project }))
}
