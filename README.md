# List app

Simple HTTP API to create, list and items.

## Requirements

- `rustup` toolchain
- `postgres` database (optional if you have docker)
- `docker` and `docker-compose` if you want to run db in container. See docker-compose.yml file.
- `psql` client to create and populate database schema.


## Step 1 - .env file

Rename `.env-sample` to `.env` file update values as needed.


## Step 2 - Database

If you have a postgres running on your machine, create a database `listlist` with user `listlist` and password `listlist`

If you want to run postgres in container, bring it up with docker-compose. Make sure pg connection settings in `.env` file correct.

Connect to postgres and create schema.

```
psql -h 127.0.0.1 -p 5432 -U listlist listlist < database.sql
```


## Step 2 - Run list app

Run app in debug mode.

```
cargo run

```


## Run app and postgres in Docker container

`Dockefile` will try to copy the release build of the app.

```
# build app in release profile.
cargo build --release

# bring up postgres and web app with docker compose file.
# If you are running swarm mode, resource limits will be used. 
docker-compose up
```