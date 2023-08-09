use super::all::Feature;
use axum::{Extension, Json, response::IntoResponse, http::StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::SqlitePool;

#[derive(Deserialize)]
pub struct ToggleFeatureDto {
    pub id: String,
}

#[derive(Serialize)]
pub struct ToggleFeatureResponse {
    new_state: bool
}

pub enum ToggleFeatureErr {
    CouldNotFetchFromDatabase    
}

impl IntoResponse for ToggleFeatureErr {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            ToggleFeatureErr::CouldNotFetchFromDatabase => (StatusCode::INTERNAL_SERVER_ERROR, "Could not fetch feature info from the database")
        };
        let body = Json(json!({
            "error": error_message
        }));
        (status, body).into_response()
    }
}

pub async fn toggle(
    Extension(pool): Extension<SqlitePool>,
    axum::extract::Json(data): Json<ToggleFeatureDto>,
) -> Result<Json<ToggleFeatureResponse>, ToggleFeatureErr> {
    let feature = match sqlx::query_as!(Feature, "SELECT * FROM feature WHERE id = ?;", data.id)
        .fetch_one(&pool)
        .await {
            Ok(feat) => feat,
            Err(_) => {
                return Err(ToggleFeatureErr::CouldNotFetchFromDatabase);
            }
        };

    let new_state = !feature.active;

    let feature = sqlx::query_as!(Feature, "UPDATE feature SET active = ? WHERE id = ? RETURNING *;", new_state, data.id)
        .fetch_one(&pool)
        .await
        .unwrap();

    Ok(Json(ToggleFeatureResponse { new_state: feature.active }))
}
