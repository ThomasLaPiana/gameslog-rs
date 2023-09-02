# Gameslog

A simple CLI and Webserver for keeping track of game backlogs.

__Note:__

The main purpose of this project is to learn about Rust and its ecosystem, with a focus on the following areas:

* Building backend services (Axum)
* Persisting data via SQLite (sqlx)
* Building CLIs (Clap)
* Build Systems/Publishing (Cargo)
* Testing
* Front-End (HTMX/Askama)

## Requirements

1. Cargo
2. npm

## Usage

The first thing to do to get up and running is to `git clone` the repo and make sure you have `cargo` installed.

From the root of the directory, run `cargo run` to see the `help` output of the CLI. To get the weberver going, run `cargo run webserver`

Games can be added one of two ways:

1. Via the REST API
1. Via the CLI

## Developer Notes

### Database Migrations (To be changed)

1. Run `sea-orm-cli migrate generate <migration_name>` from the root dir
1. Import the new migration file into `migration/src/lib.rs` using `mod` and add it to the migration Vector in chronological order
1. Run `sea-orm-cli migrate up` from the root dir
1. Run `sea-orm-cli generate entity -o entity/src` from the root dir
1. Update `api/src/models.rs` to match the new entities
