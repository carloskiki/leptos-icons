# Build

This crate is the **generator** of the `leptos-icons` library living alongside this crate.

## Usage

Trigger a build with

    cargo run

This will use the downloaded icon packages from the previous run, updating them when necessary, and generate the `leptos-icons` library crate.

Note that this is the default as this may greatly reduces the runtime of this crate.

If you want to run a clean build, removing previously downloaded content, preferred when generating a new release, use

    cargo run -- --clean

This is the only possible argument right now. You can always check for other arguments with

    cargo run -- --help

## Notes

The library crate is not generated completely from scratch. The following files and directories are touched.

| Path       | Changes  |
| ---        | ---      |
| src/*      | Replaced |
| Cargo.toml | Replaced |
| README.md  | Replaced |
| ICONS.md   | Replaced |
