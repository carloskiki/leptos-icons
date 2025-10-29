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
//! use leptos_icons::Icon;
//!
//! # #[cfg(target_arch = "wasm32")]
//! let _ = view! {
//!     <Icon icon=icondata::BsFolder />
//! };
//! ```
//! [__Complete examples__](https://github.com/carloskiki/leptos-icons/tree/main/examples) are available on github.

use leptos::{prelude::*, svg};

/// Merges the icon's style with an optional user-provided style.
fn merge_styles(user_style: Option<String>, icon_style: Option<&'static str>) -> Option<String> {
    match (user_style, icon_style) {
        (Some(a), Some(b)) => Some(format!("{b} {a}")),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b.to_string()),
        _ => None,
    }
}

/// Macro to apply common SVG attributes from an icon.
macro_rules! apply_icon_attrs {
    ($element:expr, $icon:expr) => {
        $element
            .attr("x", $icon.x)
            .attr("y", $icon.y)
            .attr("viewBox", $icon.view_box)
            .attr("stroke-linecap", $icon.stroke_linecap)
            .attr("stroke-linejoin", $icon.stroke_linejoin)
            .attr("stroke-width", $icon.stroke_width)
            .attr("stroke", $icon.stroke)
            .attr("fill", $icon.fill.unwrap_or("currentColor"))
            .attr("role", "graphics-symbol")
    };
}

/// The Icon component.
#[component]
pub fn Icon(
    /// The icon to render.
    #[prop(into)]
    icon: Signal<icondata_core::Icon>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(into, optional)] width: MaybeProp<String>,
    #[prop(into, optional)] height: MaybeProp<String>,
) -> impl IntoView {
    move || {
        let icon = icon.get();
        let svg_element = svg::svg()
            .style(merge_styles(style.get(), icon.style))
            .attr("width", width.get().unwrap_or_else(|| "1em".to_string()))
            .attr("height", height.get().unwrap_or_else(|| "1em".to_string()));

        apply_icon_attrs!(svg_element, icon).child(svg::InertElement::new(icon.data))
    }
}

/// Creates a `<symbol>` with the icon.
///
/// # Example
/// ```rust
/// use leptos::prelude::*;
/// use leptos_icons::Symbol;
///
/// #[component]
/// fn MyComponent() -> impl IntoView {
///     view! {
///         <Symbol icon=icondata::OcAlertSm id="my_icon"/>
///         <svg viewBox="0 0 1024 1024" width="1em" height="1em">
///             <use href="#my_icon" />
///         </svg>
///     }
/// }
/// ```
#[component]
pub fn Symbol(
    /// Id of the symbol for later refernce.
    /// Used as the `href` property in a `<use>` element.
    #[prop(into)]
    id: String,
    /// The icon to render.
    #[prop(into)]
    icon: Signal<icondata_core::Icon>,
    #[prop(into, optional)] style: MaybeProp<String>,
) -> impl IntoView {
    move || {
        let icon = icon.get();
        let sym = svg::symbol()
            .style(merge_styles(style.get(), icon.style))
            .attr("id", id.clone());

        let sym = apply_icon_attrs!(sym, icon).child(svg::InertElement::new(icon.data));

        svg::svg().style("display: none;").child(sym)
    }
}
