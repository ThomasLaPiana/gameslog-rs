use axum::{routing::get, Json, Router};
use serde_json::{json, Value};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

async fn root() -> Json<Value> {
    Json(json!({
        "data": "Congratulations! You've reached the Gameslog server!"
    }))
}
async fn list_games() -> Json<Value> {
    Json(json!({
        "data": "Here are some games!"
    }))
}

async fn get_game() -> Json<Value> {
    Json(json!({
        "data": "Here is a game!"
    }))
}

async fn create_game() -> Json<Value> {
    Json(json!({
        "data": "Created a game!"
    }))
}

async fn delete_game() -> Json<Value> {
    Json(json!({
        "data": "Deleted a game!"
    }))
}

#[tokio::main]
async fn main() {
    // Set tracing for logs
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let app = Router::new()
        // Add Routes
        .route("/", get(root))
        .route(
            "/games",
            get(list_games).post(create_game).delete(delete_game),
        )
        .route("/games/:id", get(get_game))
        // Add Logging
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
