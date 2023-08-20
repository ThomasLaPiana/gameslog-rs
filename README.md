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

### Database Migrations

The flow for Sea-ORM is as follows:
1.Create a new Migration file
2. Write the Migration by hand with the desired changes
3. Generate the Entity
