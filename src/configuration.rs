/// Handle file-based application configuration
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub url: String,
    pub port: u16,
}

/// Load a TOML file into a Config struct
pub fn load_config(config_file_path: &Path) -> Config {
    let raw_config = fs::read_to_string(config_file_path).expect("File could not be read!");
    let parsed_config: Config = toml::from_str(&raw_config).unwrap();
    println!("Parsed Config:\n  {:?}", parsed_config);

    Config {
        url: parsed_config.url,
        port: parsed_config.port,
    }
}
