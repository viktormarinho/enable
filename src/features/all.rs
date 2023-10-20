use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Serialize;
use serde_json::json;
use sqlx::SqlitePool;

use crate::models::feature::EnvironmentFeature;

#[derive(Serialize)]
pub struct AllFeaturesResponse {
    features: Vec<EnvironmentFeature>,
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
    env_id: String,
) -> Result<Vec<EnvironmentFeature>, AllFeaturesErr> {
    let features = EnvironmentFeature::get_by_environment_id(env_id, pool).await;

    if features.is_err() {
        return Err(AllFeaturesErr::CouldNotFetchFromDatabase);
    }

    Ok(features.unwrap())
}

pub async fn all(
    Extension(pool): Extension<SqlitePool>,
    Path(env_id): Path<String>,
) -> Result<(StatusCode, Json<AllFeaturesResponse>), AllFeaturesErr> {
    let features = fetch_all_features(&pool, env_id).await?;

    Ok((StatusCode::OK, Json(AllFeaturesResponse { features })))
}

mod tests {

    #[tokio::test]
    async fn fetch_all_features_work() {
        use crate::{db::get_pool, features::all::fetch_all_features};
        let pool = get_pool().await;
        let features = fetch_all_features(&pool,  String::from("dev")).await;
        assert!(features.is_ok());
    }
}
