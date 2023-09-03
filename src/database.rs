use sqlx;
use sqlx::{migrate::MigrateDatabase, sqlite, ConnectOptions, Connection};
use std::str::FromStr;

const DATABASE_URL: &str = "sqlite://gameslog.sqlite?mode=rwc";

/// Create the database connection and test it
pub async fn get_db_connection() -> sqlx::Result<sqlite::SqliteConnection> {
    let mut db_conn = sqlite::SqliteConnectOptions::from_str(DATABASE_URL)?
        .connect()
        .await?;

    // Test the db connection
    db_conn.ping().await.expect("Database Ping Failed!");

    Ok(db_conn)
}

/// Run any pending migrations
pub async fn run_migrations() -> Result<(), sqlx::Error> {
    let mut db_connection = get_db_connection().await?;
    sqlx::migrate!("src/migrations")
        .run(&mut db_connection)
        .await?;
    Ok(())
}

/// Used in tests
#[allow(dead_code)]
pub fn reset_database() -> Result<(), sqlx::Error> {
    sqlx::Sqlite::drop_database(DATABASE_URL);
    Ok(())
}
