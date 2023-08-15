use clap::{arg, Arg, Command};

pub fn cli() -> Command {
    // Gameslog entrypoint
    Command::new("gameslog")
        .about("A CLI to interact with the Gameslog webserver.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
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
                .arg(arg!(name: [NAME]))
                .arg(arg!(platform: [PLATFORM])),
        )
        // Delete
        .subcommand(
            Command::new("delete")
                .about("Delete a Game")
                .arg_required_else_help(true)
                .arg(arg!(name: [NAME])),
        )
}
