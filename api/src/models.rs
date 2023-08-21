use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub enum Platform {
    Switch,
    Gamepass,
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Platform::Switch => write!(f, "Switch"),
            Platform::Gamepass => write!(f, "Gamepass"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub title: String,
    pub platform: Platform,
}
