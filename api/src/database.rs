use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing::log;

const DATABASE_URL: &str = "sqlite://gameslog.sqlite?mode=rwc";

/// Create the database connection and test it
pub async fn get_db_connection() -> DatabaseConnection {
    let mut opt = ConnectOptions::new(DATABASE_URL);
    opt.max_connections(100)
        .min_connections(5)
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    let db = Database::connect(opt).await.unwrap();

    // Test the db connection
    assert!(db.ping().await.is_ok());

    db
}

/// Run any pending migrations
pub async fn run_migrations() {
    let db_connection = get_db_connection().await;
    Migrator::up(&db_connection, None).await.unwrap();
}
