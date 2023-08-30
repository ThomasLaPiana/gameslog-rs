use clap::ArgMatches;
use reqwest;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GameInfo {
    pub title: String,
    pub platform: String,
}

/// View command
pub fn view_command() -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get("http://localhost:8080/games".to_string())
        .send()?;

    let status = response.status();
    let body = response.text()?;
    println!("Status: {}\nResponse Body: {:?}", status, body);
    return Ok(body);
}

/// Send a new Game to the webserver
pub fn add_command(new_game: GameInfo) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .post("http://localhost:8080/games".to_string())
        .json(&new_game)
        .send()?;

    let status = response.status();
    let body = response.text()?;
    println!("Status: {}\nResponse Body: {:?}", status, body);
    return Ok(body);
}

/// Delete a Game from the webserver
pub fn delete_command(_full_url: String, _sub_matches: &ArgMatches) {}
