# Leptos-Icons
Add icons from popular icon libraries into your leptos projects. Every icon is packaged as its own cargo feature to reduce build times.

- Please note that this crate is in very early developpement and may include [bugs](#contributing).
- This crate is **heavily** inspired by the [solidjs-icons](https://github.com/x64Bits/solid-icons) library.

## Table of Contents

- [Usage](#usage)
- [Included Icons](#icon-packages)
- [Contributing](#contributing)

## Usage
To use this crate, it is currently required to use github linking as it is not yet published to crates.io.

```toml
leptos-icons = { git = "https://github.com/Carlosted/leptos-icons.git" }
```

It is then possible to add Icons by their feature names.

## Icon Packages

| Icon Library                                                       | License                                                                 | Short Name |
| ------------------------------------------------------------------ | ----------------------------------------------------------------------- | ---- |
| [Ant Design Icons](https://github.com/ant-design/ant-design-icons) | [MIT](https://opensource.org/licenses/MIT)                              | "ai" |
| [Bootstrap Icons](https://github.com/twbs/icons)                   | [MIT](https://opensource.org/licenses/MIT)                              | "bs" |
| [BoxIcons](https://github.com/atisawd/boxicons)                    | [CC BY 4.0 License](https://creativecommons.org/licenses/by/4.0/)       | "bi" |
| [Feather](https://feathericons.com/)                               | [MIT](https://github.com/feathericons/feather/blob/master/LICENSE)      | "fi" |
| [Font Awesome](https://fontawesome.com/)                           | [CC BY 4.0 License](https://creativecommons.org/licenses/by/4.0/)       | "fa" |
| [Heroicons](https://github.com/refactoringui/heroicons)            | [MIT](https://github.com/tailwindlabs/heroicons/blob/master/LICENSE)    | "hi" |
| [IcoMoon Free](https://github.com/Keyamoon/IcoMoon-Free)           | [CC BY 4.0 License](https://creativecommons.org/licenses/by/4.0/)       | "im" |
| [Ionicons](https://ionicons.com/)                                  | [MIT](https://github.com/ionic-team/ionicons/blob/master/LICENSE)       | "io" |
| [Remix Icon](https://github.com/Remix-Design/RemixIcon)            | [Apache License Version 2.0](http://www.apache.org/licenses/)           | "ri" |
| [Simple Icons](https://simpleicons.org/)                           | [CC0 1.0 Universal](https://creativecommons.org/publicdomain/zero/1.0/) | "si" |
| [Typicons](http://s-ings.com/typicons/)                            | [CC BY-SA 3.0](https://creativecommons.org/licenses/by-sa/3.0/)         | "ti" |
| [VS Code Icons](https://github.com/microsoft/vscode-codicons)      | [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/)               | "vs" |
| [Weather Icons](https://erikflowers.github.io/weather-icons/)      | [SIL OFL 1.1](http://scripts.sil.org/OFL)                               | "wi" |
| [css.gg](https://github.com/astrit/css.gg)                         | [MIT](https://opensource.org/licenses/MIT)                              | "cg" |
| [Tabler Icons](https://github.com/tabler/tabler-icons)             | [MIT](https://opensource.org/licenses/MIT)                              | "tb" |
| [Github Octicons](https://github.com/primer/octicons)              | [MIT](https://opensource.org/licenses/MIT)                              | "oc" |

## Contributing
Non-inclusive list of crucial missing features:
- Better Docs
- Arbitrary props passing
- remove useless categories (e.g. vscode-light/dark, sizes?)
- ssr optimizations?

Bugs:
- Tabler Icon's "Pagebreak" and "PageBreak" icons have the same file name.
- Icon names starting with digits
- Tracing feature ([#1][i1])
- "stroke-width" attribute

[i1]: https://github.com/Carlosted/leptos-icons/issues/1
