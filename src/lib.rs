//! [![github]](https://github.com/carloskiki/leptos-icons)&ensp;[![crates-io]](https://crates.io/crates/leptos_icons)&ensp;[![docs-rs]](crate)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! <br>
//!
//! A simple component that reactively renders an icon.
//!
//! To render icons, this crate needs to be coupled with [`icondata`](https://docs.rs/icondata/latest/icondata/),
//! which is an icon source providing over 20,000 icons.
//!
//! # Getting Started
//!
//! In your Cargo.toml, include both `leptos_icons` and `icondata`:
//!
//! ```toml
//! [dependencies]
//! leptos_icons = { version = "{crate_version}" }
//! icondata = { version = "{icondata_version}" }
//! ```
//!
//! In your leptos project, use:
//! ```
//! use leptos::prelude::*;
//! use leptos_icons::*;
//!
//! # #[cfg(target_arch = "wasm32")]
//! let _ = view! {
//!     <Icon icon=icondata::BsFolder />
//! };
//! ```
//! [__Complete examples__](https://github.com/carloskiki/leptos-icons/tree/main/examples) are available on github.

use leptos::{prelude::*, svg};

/// The Icon component.
#[component]
pub fn Icon(
    /// The icon to render.
    #[prop(into)]
    icon: Signal<icondata_core::Icon>,
    /// The width of the icon (horizontal side length of the square surrounding the icon). Defaults to "1em".
    #[prop(optional, into)]
    width: MaybeProp<String>,
    /// The height of the icon (vertical side length of the square surrounding the icon). Defaults to "1em".
    #[prop(optional, into)]
    height: MaybeProp<String>,
    /// HTML class attribute.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// HTML style attribute.
    #[prop(optional, into)]
    style: MaybeProp<String>,
) -> impl IntoView {
    move || {
        let icon = icon.get();

        svg::svg()
            .class(class.get())
            .style(match (style.get(), icon.style) {
                (Some(a), Some(b)) => Some(format!("{b} {a}")),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b.to_string()),
                _ => None,
            })
            .attr("x", icon.x)
            .attr("y", icon.y)
            // The style set by the user overrides the style set by the icon.
            // We ignore the width and height attributes of the icon, even if the user hasn't specified any.
            .attr(
                "width",
                width
                    .get()
                    .map_or_else(|| "1rem".to_string(), |width| width),
            )
            .attr(
                "height",
                height
                    .get()
                    .map_or_else(|| "1rem".to_string(), |height| height),
            )
            .attr("viewBox", icon.view_box)
            .attr("stroke-linecap", icon.stroke_linecap)
            .attr("stroke-linejoin", icon.stroke_linejoin)
            .attr("stroke-width", icon.stroke_width)
            .attr("stroke-linecap", icon.stroke_linecap)
            .attr("stroke", icon.stroke)
            .attr("fill", icon.fill.unwrap_or("currentColor"))
            .attr("role", "graphics-symbol")
            .inner_html(icon.data)
    }
}
