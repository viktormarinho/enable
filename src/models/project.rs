use super::*;

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub id: Option<String>,
    pub name: String,
    pub user_id: String,
}

impl Project {
    pub fn new(name: String, user_id: String) -> Project {
        let id = Some(gen::id());
        Project {
            name,
            user_id,
            id
        }
    }

    pub async fn save(&self, pool: &SqlitePool) -> Result<Project, sqlx::Error> {
        sqlx::query_as!(
            Project,
            "INSERT INTO project (name, user_id, id) VALUES (?, ?, ?) RETURNING *",
            self.name,
            self.user_id,
            self.id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn get(project_id: String, pool: &SqlitePool) -> Result<Project, sqlx::Error> {
        sqlx::query_as!(Project, "SELECT * FROM project WHERE id = ?", project_id)
        .fetch_one(pool)
        .await
    }

    pub async fn envs(project_id: String, pool: &SqlitePool) -> Result<Vec<Environment>, sqlx::Error> {
        sqlx::query_as!(
            Environment,
            "SELECT * FROM environment WHERE project_id = ?;",
            project_id
        )
        .fetch_all(pool)
        .await
    }

    pub async fn features(project_id: String, pool: &SqlitePool) -> Result<Vec<Feature>, sqlx::Error> {
        sqlx::query_as!(
            Feature,
            "SELECT * FROM feature WHERE project_id = ?;",
            project_id
        )
        .fetch_all(pool)
        .await
    }
}