use sqlx::{SqlitePool, migrate::MigrateDatabase};

async fn create_db() -> SqlitePool{
    let db_url = std::env::var("DATABASE_URL").unwrap_or(
        String::from("sqlite:enable.db?mode=rwc")
    );
    
    tracing::info!("Boostraping with DATABASE_URL {}", &db_url);

    let db_exists = sqlx::Sqlite::database_exists(&db_url).await.unwrap();

    if !db_exists {
        tracing::info!("Database does not exist, creating one...");
        sqlx::Sqlite::create_database(&db_url).await.unwrap();
    }

    let pool = SqlitePool::connect(&db_url).await.unwrap();

    tracing::info!("Checking and applying pending migrations...");
    migrate(&pool).await;
    
    return pool;
}

async fn migrate(pool: &SqlitePool) {
    let migrations = vec![
        include_str!("../migrations/1_initial.sql"),
        include_str!("../migrations/2_projects.sql"),
        include_str!("../migrations/3_creds.sql")
    ];

    for migration in migrations {
        sqlx::query(migration)
            .execute(pool)
            .await
            .expect("Setup error: Could not run migrations");
    }
}

pub async fn get_pool() -> SqlitePool {
    create_db().await
}