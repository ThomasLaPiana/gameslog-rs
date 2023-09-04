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

For development, the following additional installations are required:

* `cargo install sqlx-cli`
* `cargo install cargo-watch`

### Building & Testing

#### Database

On first run, it is necessary to create the database and run migrations using the `sqlx` CLI. This is because `sqlx` connects to the database at build time to provide checks and guarantee correctness. The first step is to create a `.env` with the following contents:

```sh
DATABASE_URL="sqlite://gameslog.sqlite?mode=rwc"
```

Next we can run the CLI command to create and migrate the database:

1. `sqlx database create`
1. `sqlx migrate run --source src/migrations/`

#### Dev Commands

Now that we've created and migrated the database, we can build the project:

```sh
cargo build
```

Spin up the server with:

```sh
cargo run webserver
```

and visit `localhost:8080/api/games` to verify everything is working! Data has already been seeded as part of the migrations.

Here is a list of helpful commands for getting started:

* `cargo check` - Runs basic static analysis without the overhead of building
* `cargo build` - Checks and builds the crate
* `cargo test` - Runs unit and integration tests. This also compiles the crate.
* `cargo fmt` - Runs unit and integration tests. This also compiles the crate.
* `cargo run` - Runs the application (the CLI in this case)

Additionally there is a helpful utility called `cargo-watch` that will automatically run a certain command when file changes are detected:

* `cargo watch -x <build|check|run|test>`

## TODOs

This is a list of additional planned features/changes

### Iterative Updates

* Refactor the CLI to support plain JSON
* Add prettier output to the CLI for JSON responses using `cursive`
* Add support for database connection pooling

### Wholly New Features

* Add a frontend/UI
* Add users and authentication?
