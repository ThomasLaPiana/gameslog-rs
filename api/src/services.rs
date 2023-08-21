use crate::database;
use crate::models::Game;
use axum::{extract, routing::delete, routing::get, routing::post, Json, Router};
use entity::game;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait};
use serde_json::{json, Value};

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
async fn list_games() -> Json<Value> {
    let db = database::get_db_connection().await;
    let games = game::Entity::find().into_json().all(&db).await.unwrap();
    Json(json!({ "data": games }))
}

/// Get a specific game by its ID
async fn get_game(extract::Path(game_id): extract::Path<String>) -> Json<Value> {
    Json(json!({ "data": format!("Retrieved: {}!", game_id) }))
}

/// Add a new game to the Gameslog
async fn create_game(extract::Json(payload): extract::Json<Game>) -> Json<Value> {
    println!("{:?}", payload);

    let new_game = game::ActiveModel {
        title: Set(payload.title.clone()),
        platform: Set(payload.platform.to_string()),
        ..Default::default()
    };
    let db = database::get_db_connection().await;
    new_game.insert(&db).await.unwrap();
    Json(json!({ "data": payload }))
}

/// Remove a game from the Gameslog
async fn delete_game(extract::Path(game_id): extract::Path<String>) -> Json<Value> {
    Json(json!({ "data": format!("Deleted: {}!", game_id) }))
}

/// Create a router with all of the endpoints used by the Games service
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
        database::run_migrations().await;

        // Build the request
        let request = Request::builder()
            .method(http::Method::POST)
            .uri("/games")
            .header("content-type", "application/json")
            .body(Body::from(
                r#"{"title": "Legend of Zelda", "platform": "Switch"}"#,
            ))
            .unwrap();

        // Send the request and get the response
        let response = app.oneshot(request).await.unwrap();

        // Load the status and body as bytes for use in assertions
        let status = response.status().clone();
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();

        assert_eq!(
            status,
            StatusCode::OK,
            "Assertion Failed due to: {:?}",
            String::from_utf8(body.to_vec()).unwrap()
        );

        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            body,
            json!({ "data": {"title": "Legend of Zelda", "platform": "Switch"} })
        );
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
