use axum::{Extension, extract, Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::SqlitePool;

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
    pub project_id: i64,
    pub name: String,
}

pub struct CreateFeatureOk {
    pub feature_id: String
}

pub enum CreateFeatureErr {
    ProjectDoesNotExist,
    FeatureAlreadyExists
}

impl IntoResponse for CreateFeatureErr {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            CreateFeatureErr::FeatureAlreadyExists => (StatusCode::BAD_REQUEST, "Feature already exists"),
            CreateFeatureErr::ProjectDoesNotExist => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error: The project does not exist")
        };
        let body = Json(json!({
            "error": error_message
        }));
        (status, body).into_response()
    }
}

pub async fn create(pool: &SqlitePool, data: CreateFeatureDto) -> Result<CreateFeatureOk, CreateFeatureErr> {
    let project = sqlx::query!("SELECT * FROM project WHERE id = ?", data.project_id)
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
        &data.name
    ).replace(" ", "_").to_lowercase();

    let existing_feature = sqlx::query!("SELECT * FROM feature WHERE id = ?", feature_id)
        .fetch_optional(pool)
        .await
        .unwrap();

    if existing_feature.is_some() {
        return Err(CreateFeatureErr::FeatureAlreadyExists);
    }

    let feature = sqlx::query!(
        "INSERT INTO feature (id, active, project_id) VALUES (?, ?, ?) RETURNING id",
        feature_id,
        false,
        data.project_id
    )
    .fetch_one(pool)
    .await
    .unwrap();

    return Ok(CreateFeatureOk { 
        feature_id: feature.id
    });
}