# Gameslog

[![CI Checks](https://github.com/ThomasLaPiana/gameslog-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/ThomasLaPiana/gameslog-rs/actions/workflows/rust.yml)

A Web App for keeping track of game backlogs.

**Note:**

The main purpose of this project is to learn about Rust and its ecosystem, with a focus on the following areas:

- Building backend services ([Axum](https://github.com/tokio-rs/axum))
- Persisting data via SQLite ([sqlx](https://github.com/launchbadge/sqlx))
- Testing ([nextest](nexte.st))
- Benchmarking ([Drill](https://github.com/fcsonline/drill))
- Front-End ([HTMX](https://htmx.org/)/[Askama](https://github.com/djc/askama)/[Tailwind](https://tailwindcss.com/))

## System Requirements

1. Cargo
1. npm
1. pnpm
1. Tailwind

## Usage

The first thing to do to get up and running is to `git clone` the repo and make sure you have `cargo` installed.

## Development

### Developer Requirements

For development, the following additional installations are required:

- `cargo install sqlx-cli`
- `cargo install rox-cli`
- `cargo install cargo-watch`

### Building & Testing

#### Database

On first run, it is necessary to create the database and run migrations using the `sqlx` CLI. This is because `sqlx` connects to the database at build time to provide checks and guarantee correctness. The first step is to create a `.env` with the following contents:

```sh
DATABASE_URL="sqlite://gameslog.sqlite?mode=rwc"
```

Next we can run the `rox` command to create and migrate the database:

```sh
rox pl setupdb
```

#### Dev Commands

Now that we've created and migrated the database, we can run the project!

```sh
rox task ws
```

This command will run the webserver and watch the local filesystem for changes, triggering a reload.

Visit `localhost:8080/` to verify everything is working! Data has already been seeded as part of the migrations.

Here are some additional commands that may be useful:

- `rox pl ci` - Runs all of the CI-style checks, including linting and testing
- `rox task tailwind` - Runs `tailwind` in `watch` mode

## Resources

The following is a list of blogs/articles/docs I used to get this all up and running:

- <https://joeymckenzie.tech/blog/templates-with-rust-axum-htmx-askama/>
