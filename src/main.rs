mod commands;
mod database;
mod games_api;
mod games_views;
mod models;
mod webserver;

use clap::{arg, Arg, Command};

pub fn cli() -> Command {
    // Gameslog entrypoint
    Command::new("gameslog")
        .about("A CLI to interact with the Gameslog webserver.")
        .arg_required_else_help(true)
        .arg(
            Arg::new("url")
                .short('u')
                .long("url")
                .help("Specify a URL")
                .default_value("http://localhost"),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .help("Specify a Port")
                .default_value("8080"),
        )
        // View
        .subcommand(Command::new("view").about("View Gameslog entries"))
        // Add
        .subcommand(
            Command::new("add")
                .about("Add a Game")
                .arg_required_else_help(true)
                .arg(arg!(title: [TITLE]))
                .arg(arg!(platform: [PLATFORM])),
        )
        // Delete
        .subcommand(
            Command::new("delete")
                .about("Delete a Game")
                .arg_required_else_help(true)
                .arg(arg!(name: [NAME])),
        )
        // Start Webserver
        .subcommand(Command::new("webserver").about("Start the Gameslog Webserver"))
}

fn main() {
    let cli_matches = cli().get_matches();

    let url = cli_matches.get_one::<String>("url").unwrap();
    println!("URL: {:?}", url);

    let port = cli_matches.get_one::<String>("port").unwrap();
    println!("Port: {:?}", port);

    let full_url = format!("{}:{}", url, port);

    match cli_matches.subcommand() {
        // View
        Some(("view", _)) => {
            println!("> Printing out the Gameslog");
            let _ = commands::view_command();
        }
        // Add
        Some(("add", sub_matches)) => {
            let title = sub_matches.get_one::<String>("title").unwrap().to_string();
            let platform = sub_matches
                .get_one::<String>("platform")
                .expect("No platform found!")
                .to_string();
            let new_game = commands::GameInfo { title, platform };
            println!("Adding a new game:\n  {:?}", new_game);
            let _ = commands::add_command(new_game);
        }
        // Delete
        Some(("delete", sub_matches)) => {
            println!("> Deleting a game.");
            commands::delete_command(full_url, sub_matches);
        }
        Some(("webserver", _)) => {
            webserver::start_webserver();
        }
        _ => (),
    }
}
