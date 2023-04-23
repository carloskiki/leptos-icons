# Lists all available commands.
list:
  just --list

# Enable nightly, add wasm target, update...
setup-rust:
  rustup default nightly
  rustup target add wasm32-unknown-unknown
  rustup update

# Install dependencies for building, running examples, profiling and possibly more...
install-tools:
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
