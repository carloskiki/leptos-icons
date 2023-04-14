# icondata

Add icons from popular icon libraries into your leptos projects. Every icon is packaged as its own cargo feature to reduce build times.

- This crate is inspired by other frameworks' icon libraries such as [solidjs-icons](https://github.com/x64Bits/solid-icons),
[yew_icons](https://github.com/finnbear/yew_icons),
[dioxus-free-icons](https://github.com/nissy-dev/dioxus-free-icons),
etc.

## Table of Contents

- [icondata](#icondata)
- [Table of Contents](#table-of-contents)
- [Usage](#usage)
- [Icon Packages](#icon-packages)
- [Contributing](#contributing)

## Usage

To use this crate, it is currently required to use Git linking, as it is not yet published to crates.io.
Use icons by specifying their feature names. For example `BsFolder` for the Bootstrap-Icons `Folder` icon.

```toml
[dependencies]
# ...
icondata = { git = "https://github.com/Carlosted/icondata.git" features = ["BsFolder"] }
```

## Icon Packages

Licenses of the icons provided through these libraries were extracted with best intent,
but must only be taken as a hint. Please check the individual icon repositories for up-to-date license information.

| Icon Library     | Version | Source                                                                                                                    | License             | Short name |
| ---              | ---     | ---                                                                                                                       | ---                 | ---        |
| Ant Design Icons | 5.3.2   | Git: <https://github.com/ant-design/ant-design-icons> - Branch: master - Commit: 7c804893b4ac698d5713b2b59f3d044eb8f5128f | MIT,                | ai         |
| Font Awesome     | 6.3.0   | Git: <https://github.com/FortAwesome/Font-Awesome> - Tag: 6.3.0                                                           | CC BY 4.0,          | fa         |
| Weather Icons    | 2.0.12  | Git: <https://github.com/erikflowers/weather-icons> - Tag: 2.0.12                                                         | SIL OFL 1.1,        | wi         |
| Feather          | 4.29.0  | Git: <https://github.com/feathericons/feather> - Tag: v4.29.0                                                             | MIT,                | fi         |
| VS Code Icons    | 0.0.32  | Git: <https://github.com/microsoft/vscode-codicons> - Tag: 0.0.32                                                         | CC BY 4.0,          | vs         |
| Bootstrap Icons  | 1.10.3  | Git: <https://github.com/twbs/icons> - Tag: v1.10.3                                                                       | MIT,                | bs         |
| BoxIcons         | 2.1.4   | Git: <https://github.com/atisawd/boxicons> - Branch: master - Commit: 9ffa9136e8681886bb7bd2145cd4098717ce1c11            | CC BY 4.0,          | bi         |
| IcoMoon Free     | unknown | Git: <https://github.com/Keyamoon/IcoMoon-Free> - Branch: master - Commit: d006795ede82361e1bac1ee76f215cf1dc51e4ca       | CC BY 4.0, GPL,     | im         |
| Ionicons         | 7.1.0   | Git: <https://github.com/ionic-team/ionicons> - Tag: v7.1.0                                                               | MIT,                | io         |
| Remix Icon       | 2.5.0   | Git: <https://github.com/Remix-Design/RemixIcon> - Tag: v2.5.0                                                            | Apache 2.0,         | ri         |
| Simple Icons     | 8.8.0   | Git: <https://github.com/simple-icons/simple-icons> - Tag: 8.8.0                                                          | CC0 1.0 Universal,  | si         |
| Typicons         | 2.1.2   | Git: <https://github.com/stephenhutchings/typicons.font> - Tag: v2.1.2                                                    | CC BY-SA 3.0,       | ti         |
| Heroicons        | 2.0.16  | Git: <https://github.com/refactoringui/heroicons> - Tag: v2.0.16                                                          | MIT,                | hi         |
| css.gg           | 2.0.0   | Git: <https://github.com/astrit/css.gg> - Tag: 2.0.0                                                                      | MIT,                | cg         |
| Tabler Icons     | 2.11.0  | Git: <https://github.com/tabler/tabler-icons> - Tag: v2.11.0                                                              | MIT,                | tb         |
| Github Octicons  | 18.3.0  | Git: <https://github.com/primer/octicons> - Tag: v18.3.0                                                                  | MIT,                | oc         |

## Contributing

Contributions are more than welcomed!
Do not hesitate add icon libraries, features, etc.
