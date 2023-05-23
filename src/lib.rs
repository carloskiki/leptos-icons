//! [![github]](https://github.com/Carlosted/leptos-icons)&ensp;[![crates-io]](https://crates.io/crates/leptos_icons)&ensp;[![docs-rs]](crate)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! <br>
//!
//! Add icons from popular icon libraries into your leptos projects. Every icon is packaged as its own cargo feature to reduce build times.
//!
//! Every enum variant has a corresponding feature name, that enables it.
//!
//! ### Example
//! In Cargo.toml, include:
//! ```toml
//! [dependencies]
//! # ...
//! leptos_icons = { version = "{crate_version}", features = ["BsFolder"] }
//! ```
//! In your leptos project, use:
//! ```
//! view! { cx,
//!     <LeptosIcon icon=BsIcon::BsFolder />
//! }
//! ```
//! To see a complete and working example, take a look at the [examples directory](https://github.com/Carlosted/leptos-icons/tree/main/examples) on github.
pub use icondata::*;

/// The Icon component.
#[leptos::component]
pub fn Icon(
    cx: leptos::Scope,
    /// The icon to show.
    #[prop(into)]
    icon: Icon,
    /// The width of the icon (horizontal side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional)]
    width: Option<String>,
    /// The height of the icon (vertical side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional)]
    height: Option<String>,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: Option<String>,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: Option<String>,
) -> impl leptos::IntoView {
    let icon = IconData::from(icon);

    let mut svg = leptos::svg::svg(cx);
    if let Some(classes) = class {
        svg = svg.classes(classes);
    }
    svg = match (style, icon.style) {
        (Some(a), Some(b)) => svg.attr("style", format!("{a} {b}")),
        (Some(a), None) => svg.attr("style", a),
        (None, Some(b)) => svg.attr("style", b),
        (None, None) => svg,
    };
    if let Some(x) = icon.x {
        svg = svg.attr("x", x);
    }
    if let Some(y) = icon.y {
        svg = svg.attr("x", y);
    }
    svg = svg.attr(
        "width",
        leptos::Attribute::String(match (width, icon.width) {
            (Some(a), Some(_b)) => std::borrow::Cow::from(a),
            (Some(a), None) => std::borrow::Cow::from(a),
            (None, Some(_b)) => std::borrow::Cow::from("1em"),
            (None, None) => std::borrow::Cow::from("1em"),
        }),
    );
    svg = svg.attr(
        "height",
        leptos::Attribute::String(match (height, icon.height) {
            (Some(a), Some(_b)) => std::borrow::Cow::from(a),
            (Some(a), None) => std::borrow::Cow::from(a),
            (None, Some(_b)) => std::borrow::Cow::from("1em"),
            (None, None) => std::borrow::Cow::from("1em"),
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
    leptos::IntoView::into_view(svg, cx)
}
