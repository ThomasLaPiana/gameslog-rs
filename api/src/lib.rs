use tower_http::trace::{self, TraceLayer};
use tracing::log;
use tracing::Level;
mod models;
mod services;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

/// Create the database connection and test it
async fn get_db_connection() -> DatabaseConnection {
    let mut opt = ConnectOptions::new("sqlite://gameslog.sqlite?mode=rwc");
    opt.max_connections(100)
        .min_connections(5)
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    let db = Database::connect(opt).await.unwrap();

    // Test the db connection
    assert!(db.ping().await.is_ok());

    db
}

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    let _db_connection = get_db_connection().await;
    // Set tracing for logs
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let app = services::create_games_router()
        // Add Logging
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}")
    }
}
