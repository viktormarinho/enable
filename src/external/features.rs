use axum::{
    extract::{Path, Query},
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    routing::get,
    Extension, Json, Router, TypedHeader,
};
use serde::{Serialize, Deserialize};
use sqlx::SqlitePool;

#[derive(Serialize)]
struct FeatureState {
    active: bool,
}

#[derive(Deserialize)]
pub struct EnvQuery {
    env_id: String
}

async fn get_feature_state(
    Extension(pool): Extension<SqlitePool>,
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    Path(id): Path<String>,
    Query(EnvQuery { env_id }): Query<EnvQuery>
) -> Result<Json<FeatureState>, StatusCode> {
    let token = authorization.token();
    let credential = sqlx::query!(r#"SELECT * FROM credentials WHERE token = ?;"#, token)
        .fetch_optional(&pool)
        .await
        .unwrap();

    if credential.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // this is going to change for sure
    let feature = sqlx::query!(r#"SELECT active FROM environment_feature WHERE feature_id = ? AND environment_id = ?;"#, id, env_id)
        .fetch_optional(&pool)
        .await
        .unwrap();

    if feature.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    let active = feature.unwrap().active;

    Ok(Json(FeatureState { active }))
}

pub fn features() -> Router {
    Router::new().route("/:id", get(get_feature_state))
}
