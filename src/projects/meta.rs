use axum::{Extension, extract::Path, Json};
use serde::Serialize;
use sqlx::SqlitePool;

use crate::models::{Project, Environment};


#[derive(Serialize)]
pub struct ProjectMeta {
    project: Project,
    envs: Vec<Environment>   
}

#[derive(Serialize)]
pub enum MetadataErr {
    CouldNotFetchFromDatabase
}

pub async fn meta(Extension(pool): Extension<SqlitePool>, Path(project_id): Path<String>) -> Result<Json<ProjectMeta>, Json<MetadataErr>> {
    let project = sqlx::query_as!(Project, "SELECT * FROM project WHERE id = ?", project_id)
        .fetch_one(&pool)
        .await
        .map_err(|_| Json(MetadataErr::CouldNotFetchFromDatabase))?;

    let envs = sqlx::query_as!(Environment, "SELECT * FROM environment WHERE project_id = ?", project_id)
        .fetch_all(&pool)
        .await
        .map_err(|_| Json(MetadataErr::CouldNotFetchFromDatabase))?;

    Ok(Json(ProjectMeta { project, envs }))
}