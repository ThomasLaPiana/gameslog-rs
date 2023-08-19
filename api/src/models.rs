use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub enum Platform {
    Switch,
    Gamepass,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    name: String,
    platform: Platform,
}
