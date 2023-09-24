mod database;
mod games_api;
mod games_views;
mod models;
mod webserver;

fn main() {
    webserver::start_webserver();
}
