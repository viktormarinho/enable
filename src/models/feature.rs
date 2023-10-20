pub use super::*;

#[derive(Serialize, Deserialize)]
pub struct Feature {
    pub id: String,
    pub project_id: String,
}

impl Feature {
    pub fn new(id: String, project_id: String) -> Feature {   
        Feature {
            id,
            project_id
        }
    }

    pub async fn save(&self, pool: &SqlitePool) -> Result<Feature, sqlx::Error> {
        sqlx::query_as!(
            Feature,
            "INSERT INTO feature (id, project_id) VALUES (?, ?) RETURNING *",
            self.id,
            self.project_id
        )
        .fetch_one(pool)
        .await
    }
}

#[derive(Serialize, Deserialize)]
pub struct EnvironmentFeature {
    pub id: String,
    pub active: bool,
    pub feature_id: String,
    pub environment_id: String,
}

impl EnvironmentFeature {
    pub async fn get(id: String, pool: &SqlitePool) -> Result<EnvironmentFeature, sqlx::Error> {
        sqlx::query_as!(EnvironmentFeature, "SELECT * FROM environment_feature WHERE id = ?;", id)
        .fetch_one(pool)
        .await
    }

    pub async fn get_by_environment_id(env_id: String, pool: &SqlitePool) -> Result<Vec<EnvironmentFeature>, sqlx::Error> {
        sqlx::query_as!(
            EnvironmentFeature,
            r#"
                SELECT *
                FROM environment_feature
                WHERE environment_id = ?;
            "#,
            env_id,
        )
        .fetch_all(pool)
        .await
    }

    pub async fn toggle(&self, pool: &SqlitePool) -> Result<EnvironmentFeature, sqlx::Error> {
        let new_state = !self.active;
        sqlx::query_as!(
            EnvironmentFeature, 
            "UPDATE environment_feature SET active = ? WHERE id = ? RETURNING *;", 
            new_state, 
            self.id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn delete_in_all_envs(self, pool: &SqlitePool) -> Result<Self, sqlx::Error> {
        let fid = self.feature_id.clone();
        sqlx::query!("DELETE FROM environment_feature WHERE feature_id = ?", fid)
            .execute(pool)
            .await
            .map(|_| ())?;

        let fid = self.feature_id.clone();
        sqlx::query!("DELETE FROM feature WHERE id = ?", fid)
            .execute(pool)
            .await
            .map(|_| ())?;

        Ok(self)
    }
}
