# SQLite Custom DateTime Datatype Implementation

This repository contains a demonstration for the custom SQLite datetime implementation for the Personal Data Aquisition Device CS46X.

The Purpose of this implementation is to achieve a timing resolution of at least `0.2ms` to meet project specifications.

I've gone to the length of setting up a docker development container for my own learning.

## Setup Instructions
### Docker Dev Container
#### Requirements:
- Some form of linux shell
- make
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

### Local Rust installation
#### Inside `/crate` run:
```sh
cargo run
```

## TODO List
- [ ] Create array of mock entries
- [ ] Create Sensor_Session_Data table
- [ ] Fill with entries
- [ ] Print table
- [ ] Get entries from table
- [ ] Print Entries