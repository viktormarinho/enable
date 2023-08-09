use axum::{
    extract::Path,
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    routing::get,
    Extension, Json, Router, TypedHeader,
};
use serde::Serialize;
use sqlx::SqlitePool;

#[derive(Serialize)]
struct FeatureState {
    active: bool,
}

async fn get_feature_state(
    Extension(pool): Extension<SqlitePool>,
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    Path(id): Path<String>,
) -> Result<Json<FeatureState>, StatusCode> {
    let token = authorization.token();
    let credential = sqlx::query!(r#"SELECT * FROM credentials WHERE token = ?;"#, token)
        .fetch_optional(&pool)
        .await
        .unwrap();

    if credential.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let feature = sqlx::query!(r#"SELECT active FROM feature WHERE id = ?;"#, id)
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
