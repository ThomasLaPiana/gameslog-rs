use serde::Deserialize;

#[derive(Deserialize)]
pub enum Platform {
    Switch(String),
    Gamepass(String),
}

#[derive(Deserialize)]
pub struct Game {
    name: String,
    platform: Platform,
}
