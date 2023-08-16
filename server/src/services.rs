use crate::models;
use axum::{extract, routing::get, Json, Router};
use serde_json::{json, Value};

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

async fn create_game(extract::Json(payload): extract::Json<serde_json::Value>) -> Json<Value> {
    println!("{:?}", payload);
    Json(json!({
        "data": "Created a game!"
    }))
}

async fn delete_game() -> Json<Value> {
    Json(json!({
        "data": "Deleted a game!"
    }))
}

pub fn create_games_router() -> Router {
    let router = Router::new()
        // Add Routes
        .route("/", get(root))
        .route(
            "/games",
            get(list_games).post(create_game).delete(delete_game),
        )
        .route("/games/:id", get(get_game));
    router
}
