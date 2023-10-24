/// These models need to stay in sync with the Sea ORM 'entities'
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Game {
    pub title: String,
    pub platforms: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GameResponse {
    pub id: String,
    pub title: String,
    pub platforms: String,
}
