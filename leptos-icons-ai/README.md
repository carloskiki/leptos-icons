# Leptos-Icons-ai

Add icons from the Ant Design Icons library into your leptos projects. Every icon is packaged as its own cargo feature to reduce build times.

## Table of Contents

- [Leptos-Icons-ai](#leptos-icons-ai)
  - [Table of Contents](#table-of-contents)
  - [Usage](#usage)
  - [Contributing](#contributing)

## Usage

To use this crate, it is currently required to use Git linking, as it is not published to crates.io.
Use icons by specifying their feature names. It is recommended to use the main crate `leptos-icons` instead.

```toml
[dependencies]
# ...
leptos-icons-ai = { git = "https://github.com/Carlosted/leptos-icons.git" features = ["..."] }
```

## Contributing

Contributions are more than welcomed!
Do not hesitate add icon libraries, features, etc.
