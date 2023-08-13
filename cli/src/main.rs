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
    println!("Parsed File:\n  {:?}", file_configuration);

    let configuration = configuration::combine_file_and_args(args, file_configuration);

    let url = format!(
        "{}:{}",
        configuration.url.unwrap(),
        configuration.port.unwrap()
    );

    // Send a request
    let response = reqwest::blocking::get(url).expect("HTTP Request failed!");

    // Show the user the output
    println!("Status: {:?}", response.status());
    println!("Data: {:?}", response.text().unwrap());
}
