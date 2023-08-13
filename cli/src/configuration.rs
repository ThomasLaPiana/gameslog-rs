/// Handle file-based application configuration
use crate::cli;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub url: Option<String>,
    pub port: Option<u16>,
}

pub fn read_file(config_file_path: &Path) -> String {
    fs::read_to_string(config_file_path).expect("File could not be read!")
}

/// Load a TOML file into a Config struct
pub fn load_config(raw_config: String) -> Config {
    let parsed_config: Config = toml::from_str(&raw_config).unwrap();

    Config {
        url: parsed_config.url,
        port: parsed_config.port,
    }
}

/// Combine the CLI and File-based configuration into a single object
/// The CLI takes precedence over the file.
pub fn combine_file_and_args(args: cli::Args, file: Config) -> Config {
    let url = if args.url.is_some() {
        args.url.unwrap()
    } else if file.url.is_some() {
        file.url.unwrap()
    } else {
        panic!("No URL value provided!")
    };

    let port = if args.port.is_some() {
        args.port.unwrap()
    } else if file.port.is_some() {
        file.port.unwrap()
    } else {
        panic!("No Port value provided!")
    };

    Config {
        url: Some(url),
        port: Some(port),
    }
}

#[test]
fn test_load_full_config() {
    let raw_config = String::from(
        r#"
    url = "testurl"
    port = 8083
    "#,
    );
    let test_parsed_config = load_config(raw_config);
    assert_eq!(test_parsed_config.url.unwrap(), String::from("testurl"));
    assert_eq!(test_parsed_config.port.unwrap(), 8083);
}

#[test]
fn test_load_partial_config() {
    let raw_config = String::from(
        r#"
    url = "testurl"
    "#,
    );
    let test_parsed_config = load_config(raw_config);
    assert_eq!(test_parsed_config.url.unwrap(), String::from("testurl"));
    assert!(test_parsed_config.port.is_none());
}

#[test]
fn test_args_override_file() {
    let dummy_args = Args {
        url: Some("args".to_string()),
        port: Some(1234),
        config_file: "args".to_string(),
    };

    let dummy_file = configuration::Config {
        url: Some("file".to_string()),
        port: Some(4321),
    };

    let test_configuration = combine_file_and_args(dummy_args, dummy_file);
    assert_eq!(test_configuration.url.unwrap(), "args");
    assert_eq!(test_configuration.port.unwrap(), 1234);
}

#[test]
fn test_no_args_configuration() {
    let dummy_args = Args {
        url: None,
        port: None,
        config_file: "args".to_string(),
    };

    let dummy_file = configuration::Config {
        url: Some("file".to_string()),
        port: Some(4321),
    };

    let test_configuration = combine_file_and_args(dummy_args, dummy_file);
    assert_eq!(test_configuration.url.unwrap(), "file");
    assert_eq!(test_configuration.port.unwrap(), 4321);
}
