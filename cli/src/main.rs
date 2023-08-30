mod cli;
mod commands;

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
        _ => (),
    }
}
