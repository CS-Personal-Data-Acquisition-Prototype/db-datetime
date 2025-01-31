# SQLite Custom DateTime Datatype Implementation

This repository contains a demonstration for the custom SQLite datetime implementation for the Personal Data Aquisition Device CS46X.

The Purpose of this implementation is to achieve a timing resolution of at least `0.2ms` to meet project specifications.

I've gone to the length of setting up a docker development container for my own learning.

## Summary

Rust provides a built in way to get the epoch in ns from the system time. Using this, we can easily cast that value to an unsigned 64 bit data type and save it to an 8-byte integer column in the database.

So far, it basically seems that we'll be ignoring SQLite's built-in datetime functions and supplementing them with our own outside the database. Pretty simple.

## Setup Instructions
### Local Rust installation
#### Requirements
- Rust

#### Inside `/crate` run either:
```sh
cargo build
cargo run
```

### Docker Dev Container
#### Requirements:
- Some form of linux shell
- Make
- Functional Docker installation

#### Available from project root `/`:
```sh
# Open a shell with a rust toolset available for use
make cli

# (Re)build the project
make build

# Run the project
make run
```
#### Windows option:
```sh
# Build the Docker image
docker build .
# Build & Run the containerized application
docker run --rm --volume .\crate:/data pda-db/devcontainer /bin/sh -c "cargo build && cargo run"
```

## TODO List
- [X] Create array of mock entries
- [X] Create Sensor_Session_Data table
- [X] Fill with entries
- [ ] Get entries from table
- [ ] Print Entries