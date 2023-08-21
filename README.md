# Gameslog

A simple CLI and Webserver for keeping track of game backlogs.

__Note:__

The main purpose of this project is to learn Rust and its ecosystem, with a focus on the following areas:

* Building backend services (Axum)
* Persisting data via Databases (Sea-ORM)
* Building CLIs (Clap)
* Build Systems/Publishing (Cargo)
* Testing

## Developer Notes

### Requirements

1. Cargo
1. Sea ORM CLI (`cargo install sea-orm-cli`)

### Database Migrations

1. Run `sea-orm-cli migrate generate <migration_name>` from the root dir
1. Import the new migration file into `migration/src/lib.rs` using `mod` and add it to the migration Vector in chronological order
1. Run `sea-orm-cli migrate up` from the root dir
1. Run `sea-orm-cli generate entity -o entity/src` from the root dir
