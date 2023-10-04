use axum::{Json, Extension, response::IntoResponse, http::StatusCode};
use serde_json::json;
use sqlx::SqlitePool;

use crate::models::feature::EnvironmentFeature;

pub enum DeleteFeatureError {
    CouldNotDeleteFromDatabase
}

impl IntoResponse for DeleteFeatureError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            DeleteFeatureError::CouldNotDeleteFromDatabase => (StatusCode::BAD_REQUEST, "Could not delete feature from database")
        };
        let body = Json(json!({
            "error": error_message
        }));
        (status, body).into_response()
    }
}

pub async fn delete(Extension(pool): Extension<SqlitePool>, Json(feature): Json<EnvironmentFeature>) -> Result<Json<EnvironmentFeature>, DeleteFeatureError> {
    feature.delete_in_all_envs(&pool)
        .await
        .map_err(|e| {
            println!("error: {e}");
            DeleteFeatureError::CouldNotDeleteFromDatabase
        })
        .map(|feat| Json(feat))
} 