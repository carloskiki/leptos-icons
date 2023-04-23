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
#[cfg(feature = "Lu")]
pub use icondata_lu::WiIcon;
#[leptos::component]
pub fn LeptosIcon(
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
