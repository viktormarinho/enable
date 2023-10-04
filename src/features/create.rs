use axum::{Extension, extract, Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::SqlitePool;

use crate::models::Project;

#[derive(Serialize)]
pub struct NewFeatureResponse {
    feature_id: String
}

pub async fn new(
    Extension(pool): Extension<SqlitePool>,
    extract::Json(body): Json<CreateFeatureDto>
) -> Result<(StatusCode, Json<NewFeatureResponse>), CreateFeatureErr> {
    Ok((
        StatusCode::OK, 
        Json(NewFeatureResponse { feature_id: create(&pool, body).await?.feature_id })
    ))
}


#[derive(Deserialize)]
pub struct CreateFeatureDto {
    pub project_id: String,
    pub name: String,
}

impl CreateFeatureDto {
    async fn validate(&self, pool: &SqlitePool) -> Result<(String, String), CreateFeatureErr> {
        if self.name.len() < 3 {
            return Err(CreateFeatureErr::FeatureNameTooShort)
        }
            
        let project = sqlx::query!("SELECT * FROM project WHERE id = ?", self.project_id)
            .fetch_optional(pool)
            .await
            .unwrap();

        if project.is_none() {
            return Err(CreateFeatureErr::ProjectDoesNotExist);
        }

        let project = project.unwrap();

        let feature_id = format!(
            "{}_{}",
            &project.name,
            &self.name
        ).replace(" ", "_").to_lowercase();

        let existing_feature = sqlx::query!("SELECT * FROM feature WHERE id = ?", feature_id)
            .fetch_optional(pool)
            .await
            .unwrap();

        if existing_feature.is_some() {
            return Err(CreateFeatureErr::FeatureAlreadyExists);
        }

        Ok((feature_id, project.id))
    }
}

pub struct CreateFeatureOk {
    pub feature_id: String
}

pub enum CreateFeatureErr {
    ProjectDoesNotExist,
    FeatureAlreadyExists,
    FeatureNameTooShort
}

impl IntoResponse for CreateFeatureErr {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message, fields) = match self {
            CreateFeatureErr::FeatureAlreadyExists => (StatusCode::BAD_REQUEST, "Feature already exists", vec!["name"]),
            CreateFeatureErr::ProjectDoesNotExist => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error: The project does not exist", vec![]),
            CreateFeatureErr::FeatureNameTooShort => (StatusCode::BAD_REQUEST, "Feature name is too short", vec!["name"])
        };
        let body = Json(json!({
            "error": error_message, "fields": fields
        }));
        (status, body).into_response()
    }
}

pub async fn create(pool: &SqlitePool, data: CreateFeatureDto) -> Result<CreateFeatureOk, CreateFeatureErr> {
    let (feature_id, project_id) = data.validate(pool).await?;

    let feature = sqlx::query!(
        "INSERT INTO feature (id, project_id) VALUES (?, ?) RETURNING id",
        feature_id,
        project_id
    )
    .fetch_one(pool)
    .await
    .unwrap();

    let envs = Project::load_envs(project_id, pool)
        .await
        .unwrap();

    for env in envs {
        env.connect_feature(pool, feature_id.clone())
            .await
            .unwrap();
    }

    return Ok(CreateFeatureOk { 
        feature_id: feature.id
    });
}