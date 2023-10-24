use askama::Template;
use axum::{response::Html, response::IntoResponse, response::Response, routing::get, Router};
use hyper::StatusCode;
use tower_http::services::ServeDir;

pub fn create_game_views_router() -> Router {
    let assets_path = format!(
        "{}/assets",
        std::env::current_dir().unwrap().to_str().unwrap()
    );

    Router::new()
        // Add Routes
        .route("/", get(index))
        .route("/games", get(list_games))
        .route("/gameslog", get(gameslog))
        .nest_service("/assets", ServeDir::new(assets_path))
}

/// Base Route
async fn index() -> impl IntoResponse {
    let template = IndexTemplate {};
    HtmlTemplate(template)
}

/// Gameslog
async fn gameslog() -> impl IntoResponse {
    let template = GameslogTemplate {};
    HtmlTemplate(template)
}

/// List the games
async fn list_games() -> impl IntoResponse {
    let template = ListGamesTemplate {};
    HtmlTemplate(template)
}
#[derive(Template)]
#[template(path = "games.html")]
struct ListGamesTemplate {}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

#[derive(Template)]
#[template(path = "gameslog.html")]
struct GameslogTemplate;

/// A wrapper type that we'll use to encapsulate HTML parsed by askama into valid HTML for axum to serve.
struct HtmlTemplate<T>(T);

/// Allows us to convert Askama HTML templates into valid HTML for axum to serve in the response.
impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        // Attempt to render the template with askama
        match self.0.render() {
            // If we're able to successfully parse and aggregate the template, serve it
            Ok(html) => Html(html).into_response(),
            // If we're not, return an error or some bit of fallback HTML
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}
