use crate::database;
use crate::games_api;
use crate::games_views;
use axum::Router;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

/// Run all server setup logic and start the server
#[tokio::main(flavor="current_thread")]
async fn start() -> anyhow::Result<()> {
    println!("> Running Migrations...");
    database::run_migrations().await?;

    // Set tracing for logs
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    println!("> Building Routers...");
    let games_api = games_api::create_games_router()
        // Add Logging
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let games_views = games_views::create_game_views_router();
    let app = Router::new().merge(games_api).merge(games_views);

    // run it with hyper on localhost:3000
    println!("> Starting server!");
    axum::Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

/// Wrapper function for starting the server
pub fn start_webserver() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}")
    }
}
