# Leptos-Icons

This is the parent for the:

| Crate             | Path               | Type | Description                                      |
| ---               | ---                | ---  | ---                                              |
| builder           | /build             | bin  | Generates the `leptos-icons` library.            |
| leptos-icons      | /leptos-icons      | lib  | Actual library published on crates.io.           |
| leptos-icons-core | /leptos-icons-core | lib  | Helpers and utility functions                    |
| leptos-icons-\*   | /leptos-icons-\*   | lib  | A library containing all icons from package "\*" |

## Executing commands

This repository uses [Just](https://github.com/casey/just)

Simply call

    just

to see a list of available commands.

You may need to install `just` using

    cargo install just
