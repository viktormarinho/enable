use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Serialize;
use serde_json::json;
use sqlx::SqlitePool;

#[derive(Serialize)]
pub struct Project {
    pub id: Option<i64>,
    pub name: String,
    pub user_id: i64,
    pub feature_count: Option<i32>
}

pub enum AllProjectsErr {
    CouldNotFetchFromDatabase,
}

impl IntoResponse for AllProjectsErr {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AllProjectsErr::CouldNotFetchFromDatabase => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal error: Could not fetch from the Database",
            ),
        };
        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}

#[derive(Serialize)]
pub struct AllProjectsResponse {
    pub projects: Vec<Project>,
}

pub async fn all(
    Extension(pool): Extension<SqlitePool>,
) -> Result<(StatusCode, Json<AllProjectsResponse>), AllProjectsErr> {
    let projects = fetch_all_projects(&pool).await?;

    Ok((StatusCode::OK, Json(AllProjectsResponse { projects })))
}

pub async fn fetch_all_projects(pool: &SqlitePool) -> Result<Vec<Project>, AllProjectsErr> {
    let result = sqlx::query_as!(
        Project,
        "SELECT 
            *,
            (select COUNT(id) FROM feature f WHERE f.project_id = p.id) as feature_count
        FROM 
            project p
        "
    )
    .fetch_all(pool)
    .await;

    if result.is_err() {
        return Err(AllProjectsErr::CouldNotFetchFromDatabase);
    }

    let projects = result.unwrap();

    Ok(projects)
}
