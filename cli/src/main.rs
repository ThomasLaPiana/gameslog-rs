mod cli;
use clap::ArgMatches;
use reqwest;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct GameInfo {
    name: String,
    platform: String,
}

/// Viewing the command
fn view_command(full_url: String) {
    let client = reqwest::blocking::Client::new();
    let response = client.get(full_url).send().unwrap();
    println!("{:?}", response);
}

/// Send a new Game to the webserver
fn add_command(full_url: String, new_game: GameInfo) {
    let client = reqwest::blocking::Client::new();
    let response = client.post(full_url).form(&new_game).send().unwrap();
    println!("{:?}", response)
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
            view_command(full_url);
        }
        // Add
        Some(("add", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").unwrap().to_string();
            let platform = sub_matches
                .get_one::<String>("platform")
                .expect("No platform found!")
                .to_string();
            let new_game = GameInfo {
                name: name,
                platform: platform,
            };
            println!("Adding a new game:\n  {:?}", new_game);
            add_command(full_url, new_game);
        }
        // Delete
        Some(("delete", sub_matches)) => {
            println!("> Deleting a game.");
            delete_command(full_url, sub_matches);
        }
        _ => unreachable!(),
    }
}
