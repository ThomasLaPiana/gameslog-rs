/// These models need to stay in sync with the Sea ORM 'entities'
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub title: Option<String>,
    pub platforms: Option<String>,
}
