# Lists all available commands.
list:
  just --list

# Install dependencies for building, running examples, profiling and possibly more...
install:
  cargo install trunk
  cargo install twiggy
  cargo install cargo-expand

# Build all libraries
build:
    cd build && cargo run

# Build all libraries, forcing new downloads of icon packages
build-clean:
    cd build && cargo run -- --clean

# Serves a simple example application
serve-example:
    cd leptos-icons/examples/simple-app && trunk serve --release
