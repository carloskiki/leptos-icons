# Lists all available commands.
list:
  just --list

# Build all libraries
build:
    cd build && cargo run

# Build all libraries, forcing new downloads of icon packages
build-clean:
    cd build && cargo run -- --clean
