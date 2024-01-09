//! [![github]](https://github.com/Carlosted/leptos-icons)&ensp;[![crates-io]](https://crates.io/crates/leptos_icons)&ensp;[![docs-rs]](crate)
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
//! use leptos::*;
//! use leptos_icons::*;
//!
//! # #[cfg(target_arch = "wasm32")]
//! let _ = view! {
//!     <Icon icon=icondata::BsFolder />
//! };
//! ```
//! [__Complete examples__](https://github.com/Carlosted/leptos-icons/tree/main/examples) are available on github.

use leptos::tracing;
use leptos::SignalGet;

/// The Icon component.
#[leptos::component]
pub fn Icon(
    /// The icon to render.
    #[prop(into)]
    icon: leptos::MaybeSignal<icondata_core::Icon>,
    /// The width of the icon (horizontal side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional)]
    width: leptos::MaybeProp<leptos::TextProp>,
    /// The height of the icon (vertical side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional)]
    height: leptos::MaybeProp<leptos::TextProp>,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: leptos::MaybeProp<leptos::TextProp>,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: leptos::MaybeProp<leptos::TextProp>,
) -> impl leptos::IntoView
where
{
    let svg = move || {
        let icon = icon.get();
        let mut svg = leptos::svg::svg();
        if let Some(classes) = class.get() {
            svg = svg.classes(classes.get());
        }
        let mut svg = match (style.get(), icon.style) {
            (Some(a), Some(b)) => svg.attr("style", format!("{b} {}", a.get())),
            (Some(a), None) => svg.attr("style", a.get()),
            (None, Some(b)) => svg.attr("style", b),
            (None, None) => svg,
        };
        if let Some(x) = icon.x {
            svg = svg.attr("x", x);
        }
        if let Some(y) = icon.y {
            svg = svg.attr("x", y);
        }
        // The style set by the user overrides the style set by the icon.
        // We ignore the width and height attributes of the icon, even if the user hasn't specified any.
        svg = svg.attr(
            "width",
            leptos::Attribute::String(match (width.get(), icon.width) {
                (Some(a), _) => leptos::Oco::from(a.get()),
                _ => leptos::Oco::from("1em"),
            }),
        );
        svg = svg.attr(
            "height",
            leptos::Attribute::String(match (height.get(), icon.height) {
                (Some(a), _) => leptos::Oco::from(a.get()),
                _ => leptos::Oco::from("1em"),
            }),
        );
        if let Some(view_box) = icon.view_box {
            svg = svg.attr("viewBox", view_box);
        }
        if let Some(stroke_linecap) = icon.stroke_linecap {
            svg = svg.attr("stroke-linecap", stroke_linecap);
        }
        if let Some(stroke_linejoin) = icon.stroke_linejoin {
            svg = svg.attr("stroke-linejoin", stroke_linejoin);
        }
        if let Some(stroke_width) = icon.stroke_width {
            svg = svg.attr("stroke-width", stroke_width);
        }
        if let Some(stroke) = icon.stroke {
            svg = svg.attr("stroke", stroke);
        }
        svg = svg.attr("fill", icon.fill.unwrap_or("currentColor"));
        svg = svg.attr("role", "graphics-symbol");
        svg = svg.inner_html(icon.data);
        svg
    };
    leptos::IntoView::into_view(svg)
}
