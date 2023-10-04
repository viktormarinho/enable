pub use super::*;

#[derive(Serialize, Deserialize)]
pub struct Feature {
    pub id: String,
    pub project_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct EnvironmentFeature {
    pub id: String,
    pub active: bool,
    pub feature_id: String,
    pub environment_id: String,
}

impl EnvironmentFeature {
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
        // can check for number of rows affected! if it is more than 1 something is wrong...

        Ok(self)
    }
}
