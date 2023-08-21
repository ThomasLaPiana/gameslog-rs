mod cli;
use clap::ArgMatches;
use reqwest;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct GameInfo {
    title: String,
    platform: String,
}

/// View command
fn view_command() {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get("http://localhost:8080/games".to_string())
        .send()
        .unwrap();

    let status = response.status();
    let body = response.text().unwrap();
    println!("Status: {}\nResponse Body: {:?}", status, body);
}

/// Send a new Game to the webserver
fn add_command(new_game: GameInfo) {
    let client = reqwest::blocking::Client::new();
    let response = client
        .post("http://localhost:8080/games".to_string())
        .json(&new_game)
        .send()
        .unwrap();

    let status = response.status();
    let body = response.text().unwrap();
    println!("Status: {}\nResponse Body: {:?}", status, body);
}

/// Delete a Game from the webserver
fn delete_command(_full_url: String, _sub_matches: &ArgMatches) {}

fn main() {
    let cli_matches = cli::cli().get_matches();

    let url = cli_matches.get_one::<String>("url").unwrap();
    println!("URL: {:?}", url);

    let port = cli_matches.get_one::<String>("port").unwrap();
    println!("Port: {:?}", port);

    let full_url = format!("{}:{}", url, port);

    match cli_matches.subcommand() {
        // View
        Some(("view", _)) => {
            println!("> Printing out the Gameslog");
            view_command();
        }
        // Add
        Some(("add", sub_matches)) => {
            let title = sub_matches.get_one::<String>("title").unwrap().to_string();
            let platform = sub_matches
                .get_one::<String>("platform")
                .expect("No platform found!")
                .to_string();
            let new_game = GameInfo {
                title: title,
                platform: platform,
            };
            println!("Adding a new game:\n  {:?}", new_game);
            add_command(new_game);
        }
        // Delete
        Some(("delete", sub_matches)) => {
            println!("> Deleting a game.");
            delete_command(full_url, sub_matches);
        }
        _ => unreachable!(),
    }
}
