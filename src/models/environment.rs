pub use super::*;

#[derive(Serialize, Deserialize)]
pub struct Environment {
    pub id: Option<String>,
    pub name: String,
    pub project_id: String
}

impl Environment {
    pub async fn new(pool: &SqlitePool, name: impl Into<String>, project_id: &String) -> Result<Environment, sqlx::Error> {
        let id = gen::id();
        let name: String = name.into();
        sqlx::query_as!(
            Environment,
            r#"
            INSERT INTO environment
            (id, name, project_id)
            VALUES 
                (?, ?, ?)
            RETURNING *;
            "#,
            id,
            name,
            project_id,
        )
        .fetch_one(pool)
        .await
    }

    pub async fn connect_feature(&self, pool: &SqlitePool, feature_id: String) -> Result<String, sqlx::Error> {
        let id = gen::id();
        sqlx::query!(
            "INSERT INTO environment_feature (id, environment_id, feature_id, active) VALUES (?, ?, ?, ?) RETURNING id",
            id,
            self.id,
            feature_id,
            false
        )
        .fetch_one(pool)
        .await.map(|rec| rec.id)
    }
}