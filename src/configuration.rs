/// Handle file-based application configuration
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub url: String,
    pub port: u16,
}

pub fn read_config_file(config_file_path: &Path) -> String {
    fs::read_to_string(config_file_path).expect("File could not be read!")
}

/// Load a TOML file into a Config struct
pub fn load_config(raw_config: String) -> Config {
    let parsed_config: Config = toml::from_str(&raw_config).unwrap();
    println!("Parsed Config:\n  {:?}", parsed_config);

    Config {
        url: parsed_config.url,
        port: parsed_config.port,
    }
}

#[test]
fn test_load_config() {
    let raw_config = String::from(
        r#"
    url = "testurl"
    port = 8083
    "#,
    );
    let test_parsed_config = load_config(raw_config);
    assert_eq!(test_parsed_config.url, String::from("testurl"));
    assert_eq!(test_parsed_config.port, 8083);
}
