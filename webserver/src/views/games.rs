use axum::{response::Html, routing::get, Router};

pub fn create_game_views_router() -> Router {
    Router::new()
        // Add Routes
        .route("/games", get(list_games))
}

async fn list_games() -> Html<&'static str> {
    Html("Hello, World!")
}
