mod configuration;
use clap::Parser;
use reqwest;
use std::path::Path;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    url: Option<String>,

    #[arg(short, long, default_value = "8080")]
    port: Option<u16>,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);

    // Load the TOML file
    let config_file_path = Path::new("config.toml");
    let raw_config = configuration::read_config_file(config_file_path);
    let configuration = configuration::load_config(raw_config);

    // Send a request
    let response = reqwest::blocking::get(&configuration.url).expect("HTTP Request failed!");

    // Show the user the output
    println!("Status: {:?}", response.status());
}
