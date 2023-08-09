use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Serialize;
use serde_json::json;
use sqlx::SqlitePool;

use crate::projects::create::Project;

#[derive(Serialize)]
pub struct Feature {
    pub id: String,
    pub active: bool,
    pub project_id: i64,
}

#[derive(Serialize)]
pub struct AllFeaturesResponse {
    features: Vec<Feature>,
    project: Project,
}

pub enum AllFeaturesErr {
    CouldNotFetchFromDatabase,
}

impl IntoResponse for AllFeaturesErr {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AllFeaturesErr::CouldNotFetchFromDatabase => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal error: Could not fetch from the database",
            ),
        };
        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}

pub async fn fetch_all_features(
    pool: &SqlitePool,
    project_id: i64,
) -> Result<Vec<Feature>, AllFeaturesErr> {
    let result = sqlx::query_as!(
        Feature,
        "SELECT * FROM feature WHERE project_id = ?",
        project_id
    )
    .fetch_all(pool)
    .await;

    if result.is_err() {
        return Err(AllFeaturesErr::CouldNotFetchFromDatabase);
    }

    Ok(result.unwrap())
}

pub async fn all(
    Extension(pool): Extension<SqlitePool>,
    Path(project_id): Path<i64>,
) -> Result<(StatusCode, Json<AllFeaturesResponse>), AllFeaturesErr> {
    let features = fetch_all_features(&pool, project_id).await?;
    let project = sqlx::query_as!(Project, "SELECT * FROM project WHERE id = ?", project_id)
        .fetch_one(&pool)
        .await
        .unwrap();

    Ok((StatusCode::OK, Json(AllFeaturesResponse { features, project })))
}

mod tests {

    #[tokio::test]
    async fn fetch_all_features_work() {
        use crate::{db::get_pool, features::all::fetch_all_features};
        let pool = get_pool().await;
        let features = fetch_all_features(&pool, 0).await;
        assert!(features.is_ok());
    }
}
