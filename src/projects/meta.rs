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
    let project = Project::get(project_id.clone(), &pool)
        .await
        .map_err(|_| Json(MetadataErr::CouldNotFetchFromDatabase))?;

    let envs = Project::envs(project_id, &pool)
        .await
        .map_err(|_| Json(MetadataErr::CouldNotFetchFromDatabase))?;

    Ok(Json(ProjectMeta { project, envs }))
}