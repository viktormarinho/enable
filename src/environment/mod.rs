use axum::Router;
use axum::routing::post;

mod create {
    use axum::{Extension, Json, response::IntoResponse, http::StatusCode};
    use serde_json::json;
    use sqlx::SqlitePool;

    use crate::models::{Environment, Project};

    pub enum CreateEnvironmentError {
        CouldNotInsertInDatabase
    }

    impl IntoResponse for CreateEnvironmentError {
        fn into_response(self) -> axum::response::Response {
            let (status, error_message, fields) = match self {
                CreateEnvironmentError::CouldNotInsertInDatabase => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Error: Could not insert in database", vec![""])
            };
            let body = Json(json!({
                "error": error_message, "fields": fields
            }));
            (status, body).into_response()
        }
    }

    pub async fn create(
        Extension(pool): Extension<SqlitePool>,
        Json(data): Json<Environment>
    ) -> Result<Json<Environment>, CreateEnvironmentError> {
        let env = Environment::new(&pool, data.name, &data.project_id)
            .await
            .map_err(|_| CreateEnvironmentError::CouldNotInsertInDatabase)?;

        let features = Project::features(data.project_id, &pool)
            .await
            .map_err(|_| CreateEnvironmentError::CouldNotInsertInDatabase)?;

        for feat in features {
            env.connect_feature(&pool, feat.id)
                .await
                .map_err(|_| CreateEnvironmentError::CouldNotInsertInDatabase)?;
        }

        Ok(Json(env))
    }
}

pub fn environment() -> Router {
    Router::new()
        .route("/", post(create::create))
}