use crate::models::Game;
use axum::{extract, routing::delete, routing::get, routing::post, Json, Router};
use serde_json::{json, Value};

async fn root() -> Json<Value> {
    Json(json!({
        "data": "Congratulations! You've reached the Gameslog server!"
    }))
}

async fn health() -> Json<Value> {
    Json(json!({
        "data": "We very healthy!"
    }))
}

async fn list_games() -> Json<Value> {
    Json(json!({
        "data": "Here are some games!"
    }))
}

async fn get_game(extract::Path(game_id): extract::Path<String>) -> Json<Value> {
    Json(json!({
        "data": format!("Retrieved: {}!", game_id)
    }))
}

async fn create_game(extract::Json(payload): extract::Json<Game>) -> Json<Game> {
    println!("{:?}", payload);
    Json(payload)
}

async fn delete_game(extract::Path(game_id): extract::Path<String>) -> Json<Value> {
    Json(json!({
        "data": format!("Deleted: {}!", game_id)
    }))
}

pub fn create_games_router() -> Router {
    let router = Router::new()
        // Add Routes
        .route("/", get(root))
        .route("/health", get(health))
        .route("/games", get(list_games))
        .route("/games", post(create_game))
        .route("/games/:game_id", delete(delete_game))
        .route("/games/:game_id", get(get_game));
    router
}
