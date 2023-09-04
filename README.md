# Gameslog

A simple CLI and Webserver for keeping track of game backlogs.

__Note:__

The main purpose of this project is to learn about Rust and its ecosystem, with a focus on the following areas:

* Building backend services (Axum)
* Persisting data via SQLite (sqlx)
* Building CLIs (Clap)
* Build Systems/Publishing (Cargo, Cross)
* Testing
* Front-End (HTMX/Askama/Tailwind)

## System Requirements

1. Cargo
2. npm

## Usage

The first thing to do to get up and running is to `git clone` the repo and make sure you have `cargo` installed.

From the root of the directory, run `cargo run` to see the `help` output of the CLI. To get the weberver going, run `cargo run webserver`

Games can be added one of two ways:

1. Via the REST API
1. Via the CLI

## Development

### Developer Requirements

For development, the following additional installations are recommended and assumed:

* `cargo install sqlx-cli`
* `cargo install cargo-watch`

### Building & Testing

The following is a list of helpful commands for getting started:

* `cargo check` - Runs basic static analysis without the overhead of building
* `cargo build` - Checks and builds the crate
* `cargo test` - Runs unit and integration tests. This also compiles the crate.
* `cargo fmt` - Runs unit and integration tests. This also compiles the crate.
* `cargo run` - Runs the application (the CLI in this case)

Additionally there is a helpful utility called `cargo-watch` that will automatically run a certain command when file changes are detected. It can be installed and invoked like so:

* `cargo install cargo-watch`
* `cargo watch -x <build|check|run|test>`

### Database Migrations

1. `sqlx migrate run --source src/migrations/`

## TODOs

This is a list of additional planned features/changes

## Iterative Updates

* Refactor the CLI to support plain JSON
* Add prettier output to the CLI for JSON responses using `cursive`

## Wholly New Features

* Add users and authentication
* Add a frontend/UI
