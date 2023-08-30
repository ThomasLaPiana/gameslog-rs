use crate::database;
use crate::models::Game;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, get, post};
use axum::{extract, Json, Router};
use entity::game;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait};
use serde_json::{json, Value};

/// Create a router with all of the endpoints used by the Games service
pub fn create_games_router() -> Router {
    Router::new()
        // Add Routes
        .route("/api", get(root))
        .route("/health", get(health))
        .route("/api/games", get(list_games))
        .route("/api/games", post(create_game))
        .route("/api/games/:game_id", delete(delete_game))
        .route("/api/games/:game_id", get(get_game))
}

// Generic welcome
async fn root() -> Json<Value> {
    Json(json!({
        "data": "You've reached the Gameslog server!"
    }))
}

async fn health() -> Json<Value> {
    Json(json!({
        "data": "Feeling healthy!"
    }))
}

/// List all of the games stored in the database
async fn list_games() -> Response {
    let db = database::get_db_connection().await;
    let games = game::Entity::find().into_json().all(&db).await.unwrap();
    (StatusCode::OK, Json(json!({ "data": games }))).into_response()
}

/// Get a specific game by its ID
async fn get_game(extract::Path(game_id): extract::Path<String>) -> Response {
    let db = database::get_db_connection().await;
    let game = game::Entity::find_by_id(&game_id)
        .into_json()
        .one(&db)
        .await;

    match game {
        Ok(game) => (StatusCode::OK, Json(json!({ "data": game }))).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "data": "Fata error occured!"})),
        )
            .into_response(),
    }
}

/// Add a new game to the Gameslog
async fn create_game(extract::Json(payload): extract::Json<Game>) -> Response {
    let game_title = payload.title.clone();
    let normalized_platforms = payload
        .platforms
        .iter()
        .map(|platform| platform.to_string())
        .collect::<Vec<String>>()
        .join(",");

    let new_game = game::ActiveModel {
        title: Set(game_title),
        platforms: Set(normalized_platforms),
        ..Default::default()
    };
    let db = database::get_db_connection().await;

    match new_game.insert(&db).await {
        Ok(_) => (StatusCode::CREATED, Json(json!({ "data": payload }))).into_response(),
        Err(insertion) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "data": insertion.to_string()})),
        )
            .into_response(),
    }
}

/// Remove a game from the Gameslog
async fn delete_game(extract::Path(game_id): extract::Path<String>) -> Response {
    let db = database::get_db_connection().await;
    let result = game::Entity::delete_by_id(&game_id).exec(&db).await;

    match result {
        Ok(_) => (
            StatusCode::NO_CONTENT,
            Json(json!({
                "data": format!("Game with ID '{}' deleted!", game_id)
            })),
        )
            .into_response(),
        Err(result) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "data": result.to_string()})),
        )
            .into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use serde_json::{json, Value};
    use tower::ServiceExt; // for `oneshot` and `ready`

    #[tokio::test]
    async fn test_health() {
        let app = create_games_router();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!({ "data": "Feeling healthy!" }));
    }

    #[tokio::test]
    async fn test_create_game() {
        let app = create_games_router();
        database::reset_database().await;
        database::run_migrations().await;

        // Build the request
        let request = Request::builder()
            .method(http::Method::POST)
            .uri("/api/games")
            .header("content-type", "application/json")
            .body(Body::from(
                r#"{"title": "Legend of Zelda", "platforms": ["Switch"]}"#,
            ))
            .unwrap();

        // Send the request and get the response
        let response = app.oneshot(request).await.unwrap();

        // Load the status and body as bytes for use in assertions
        let status = response.status();
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();

        assert_eq!(
            status,
            StatusCode::CREATED,
            "Assertion Failed due to: {:?}",
            String::from_utf8(body.to_vec()).unwrap()
        );

        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            body,
            json!({ "data": {"title": "Legend of Zelda", "platforms": ["Switch"]} })
        );
    }

    #[tokio::test]
    async fn test_create_game_invalid() {
        let app = create_games_router();
        database::run_migrations().await;

        // Build the request
        let request = Request::builder()
            .method(http::Method::POST)
            .uri("/api/games")
            .header("content-type", "application/json")
            .body(Body::from(
                r#"{"title": "Legend of Zelda", "platforms": "Switch"}"#,
            ))
            .unwrap();

        // Send the request and get the response
        let response = app.oneshot(request).await.unwrap();

        // Load the status and body as bytes for use in assertions
        let status = response.status();

        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY,);
    }

    #[tokio::test]
    async fn not_found() {
        let app = create_games_router();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/does-not-exist")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert!(body.is_empty());
    }
}
