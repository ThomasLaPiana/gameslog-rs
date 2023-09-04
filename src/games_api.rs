use crate::database;
use crate::models::Game;
use ::std::collections::HashMap;
use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, post};
use axum::{extract, Json, Router};
use serde_json::{json, Value};

/// Create a router with all of the endpoints used by the Games service
pub fn create_games_router() -> Router {
    Router::new()
        // Add Routes
        .route("/api", get(root))
        .route("/api/health", get(health))
        .route("/api/games", get(list_games))
        .route("/api/games", post(create_game))
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

#[derive(Template)]
#[template(path = "game_list.html")]
struct GamesListTemplate {
    games: Vec<Game>,
}

/// List all of the games stored in the database
async fn list_games(extract::Query(params): extract::Query<HashMap<String, String>>) -> Response {
    let mut db = database::get_db_connection().await.unwrap();
    let games = sqlx::query_as!(Game, "SELECT title, platforms FROM games")
        .fetch_all(&mut db)
        .await
        .unwrap();

    if params.contains_key("html") {
        return Html(GamesListTemplate { games }.render().unwrap()).into_response();
    }

    (StatusCode::OK, Json(json!({ "data": games }))).into_response()
}

/// Get a specific game by its ID
async fn get_game(extract::Path(game_id): extract::Path<String>) -> Response {
    let mut db = database::get_db_connection().await.unwrap();
    // TODO: Make the match case insensitive
    let game = sqlx::query_as!(
        Game,
        "SELECT title, platforms FROM games where title = ?",
        game_id
    )
    .fetch_one(&mut db)
    .await;

    match game {
        Ok(game) => (StatusCode::OK, Json(json!({ "data": game }))).into_response(),
        _ => (StatusCode::OK, Json(json!({ "data": ""}))).into_response(),
    }
}

/// Add a new game to the Gameslog
async fn create_game(extract::Json(payload): extract::Json<Game>) -> Response {
    let game_title = payload.title.clone();
    let game_platforms = payload.platforms.clone();

    // TOOD: enforce uniqueness
    let mut db = database::get_db_connection().await.unwrap();
    sqlx::query!("INSERT INTO games VALUES(?, ?)", game_title, game_platforms,)
        .execute(&mut db)
        .await
        .unwrap();

    (StatusCode::CREATED, Json(json!({ "data": payload }))).into_response()
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
                    .uri("/api/health")
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
        database::reset_database().unwrap();
        database::run_migrations().await.unwrap();

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
            json!({ "data": {"title": "Legend of Zelda", "platforms": "Switch"} })
        );
    }

    #[tokio::test]
    async fn test_get_nonexistent_game() {
        let app = create_games_router();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/games/foo")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!({ "data": "" }));
    }

    #[tokio::test]
    async fn test_create_game_invalid() {
        // Create database if not exists:
        let app = create_games_router();
        database::run_migrations().await.unwrap();

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

        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY,);
    }

    #[tokio::test]
    async fn route_not_found() {
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
