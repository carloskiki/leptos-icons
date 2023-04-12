# Leptos-Icons-im

Add icons from the IcoMoon Free library into your leptos projects. Every icon is packaged as its own cargo feature to reduce build times.

## Table of Contents

- [Leptos-Icons-im](#leptos-icons-im)
  - [Table of Contents](#table-of-contents)
  - [Usage](#usage)
  - [Contributing](#contributing)

## Usage

To use this crate, it is currently required to use Git linking, as it is not published to crates.io.
Use icons by specifying their feature names. It is recommended to use the main crate `leptos-icons` instead.

```toml
[dependencies]
# ...
leptos-icons-im = { git = "https://github.com/Carlosted/leptos-icons.git" features = ["..."] }
```

## Contributing

Contributions are more than welcomed!
Do not hesitate add icon libraries, features, etc.
