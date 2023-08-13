use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub url: Option<String>,

    #[arg(short, long, default_value = "config.toml")]
    pub config_file: String,

    #[arg(short, long, default_value = "8080")]
    pub port: Option<u16>,
}
