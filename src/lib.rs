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
//! leptos-icons = { git = "https://github.com/Carlosted/leptos-icons.git" features = ["BsFolder"] }
//! ```
//! In your leptos project, use:
//! ```
//! view! { cx,
//!     <LeptosIcon icon=BsIcon::BsFolder />
//! }
//! ```
//! To see a complete and working example, take a look at the [examples directory](https://github.com/Carlosted/leptos-icons/tree/main/examples) on github.
#[cfg(feature = "Ai")]
pub use icondata_ai::AiIcon;
#[cfg(feature = "Bi")]
pub use icondata_bi::BiIcon;
#[cfg(feature = "Bs")]
pub use icondata_bs::BsIcon;
#[cfg(feature = "Cg")]
pub use icondata_cg::CgIcon;
#[cfg(feature = "Fa")]
pub use icondata_fa::FaIcon;
#[cfg(feature = "Fi")]
pub use icondata_fi::FiIcon;
#[cfg(feature = "Hi")]
pub use icondata_hi::HiIcon;
#[cfg(feature = "Im")]
pub use icondata_im::ImIcon;
#[cfg(feature = "Io")]
pub use icondata_io::IoIcon;
#[cfg(feature = "Lu")]
pub use icondata_lu::LuIcon;
#[cfg(feature = "Oc")]
pub use icondata_oc::OcIcon;
#[cfg(feature = "Ri")]
pub use icondata_ri::RiIcon;
#[cfg(feature = "Si")]
pub use icondata_si::SiIcon;
#[cfg(feature = "Tb")]
pub use icondata_tb::TbIcon;
#[cfg(feature = "Ti")]
pub use icondata_ti::TiIcon;
#[cfg(feature = "Vs")]
pub use icondata_vs::VsIcon;
#[cfg(feature = "Wi")]
pub use icondata_wi::WiIcon;
#[leptos::component]
/// The Icon component.
pub fn Icon(
    cx: leptos::Scope,
    /// The icon to show.
    #[prop(into)]
    icon: icondata_core::IconData,
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
            (Some(a), Some(_b)) => a,
            (Some(a), None) => a,
            (None, Some(_b)) => "1em".to_owned(),
            (None, None) => "1em".to_owned(),
        }),
    );
    svg = svg.attr(
        "height",
        leptos::Attribute::String(match (height, icon.height) {
            (Some(a), Some(_b)) => a,
            (Some(a), None) => a,
            (None, Some(_b)) => "1em".to_owned(),
            (None, None) => "1em".to_owned(),
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
