use super::*;

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub id: Option<String>,
    pub name: String,
    pub user_id: String,
}

impl Project {
    pub async fn load_envs(project_id: String, pool: &SqlitePool) -> Result<Vec<Environment>, sqlx::Error> {
        sqlx::query_as!(
            Environment,
            "SELECT * FROM environment WHERE project_id = ?;",
            project_id
        )
        .fetch_all(pool)
        .await
    }
}