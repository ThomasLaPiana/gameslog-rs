use axum::Router;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
mod api {
    pub mod games;
}
mod database;
mod models;
mod views {
    pub mod games;
}

/// Run all server setup logic and start the server
#[tokio::main]
async fn start() -> anyhow::Result<()> {
    println!("> Running Migrations...");
    database::run_migrations().await;

    // Set tracing for logs
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    println!("> Building Routers...");
    let games_api = api::games::create_games_router()
        // Add Logging
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let games_views = views::games::create_game_views_router();
    let app = Router::new().merge(games_api).merge(games_views);

    // run it with hyper on localhost:3000
    println!("> Starting server!");
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