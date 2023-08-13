mod cli;
mod configuration;
use clap::Parser;
use reqwest;
use std::path::Path;

fn main() {
    let args = cli::Args::parse();
    println!("Parsed Args:\n  {:?}", args);

    // Build the configuration for the application
    let config_file_path = Path::new(&args.config_file);
    let raw_config = configuration::read_file(config_file_path);
    let file_configuration = configuration::load_config(raw_config);
    let configuration = configuration::combine_file_and_args(args, file_configuration);

    // Send a request
    let response =
        reqwest::blocking::get(&configuration.url.unwrap()).expect("HTTP Request failed!");

    // Show the user the output
    println!("Status: {:?}", response.status());
}
