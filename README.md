# Gameslog

A simple CLI and Webserver for keeping track of game backlogs.

__Note:__

The main purpose of this project is to learn Rust and its ecosystem, with a focus on the following areas:

* Building backend services (Axum)
* Persisting data via Databases (Sea-ORM)
* Building CLIs (Clap)
* Build Systems/Publishing (Cargo)
* Testing

## Requirements

1. Cargo
1. Sea ORM CLI (`cargo install sea-orm-cli`)

## Usage

The first thing to do to get up and running is to `git clone` the repo and make sure you have `cargo` installed.

From the root of the directory, run `cargo run --bin gameslog` to spin up the server, create the serverfile (SQLite) and run the migrations. You're ready to start adding games!

Games can be added one of two ways:

1. Via the REST API
1. Via the CLI, which you can run with `cargo run --bin gameslog-cli -- -h`

## Developer Notes

### Database Migrations

1. Run `sea-orm-cli migrate generate <migration_name>` from the root dir
1. Import the new migration file into `migration/src/lib.rs` using `mod` and add it to the migration Vector in chronological order
1. Run `sea-orm-cli migrate up` from the root dir
1. Run `sea-orm-cli generate entity -o entity/src` from the root dir
1. Update `api/src/models.rs` to match the new entities
