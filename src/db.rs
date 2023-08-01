use sqlx::SqlitePool;

pub async fn get_pool() -> SqlitePool {
    SqlitePool::connect(&std::env::var("DATABASE_URL")
        .expect("DATABASE_URL env var not set")).await.unwrap()
}