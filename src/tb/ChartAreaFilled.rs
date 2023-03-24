#[cfg(feature = "TbChartAreaFilled")]
use leptos::*;
#[cfg(feature = "TbChartAreaFilled")]
///This icon requires the feature `TbChartAreaFilled` to be enabled.
#[component]
pub fn ChartAreaFilled(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    color: String,
    /// HTML style attribute.
    #[prop(into)]
    #[prop(optional)]
    style: String,
    /// Accessibility title.
    #[prop(into)]
    #[prop(optional)]
    title: String,
) -> impl IntoView {
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-chart-area-filled" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M20 18a1 1 0 0 1 .117 1.993l-.117 .007h-16a1 1 0 0 1 -.117 -1.993l.117 -.007h16z"
        stroke - width = "0" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M15.22 5.375a1 1 0 0 1 1.393 -.165l.094 .083l4 4a1 1 0 0 1 .284 .576l.009 .131v5a1 1 0 0 1 -.883 .993l-.117 .007h-16.022l-.11 -.009l-.11 -.02l-.107 -.034l-.105 -.046l-.1 -.059l-.094 -.07l-.06 -.055l-.072 -.082l-.064 -.089l-.054 -.096l-.016 -.035l-.04 -.103l-.027 -.106l-.015 -.108l-.004 -.11l.009 -.11l.019 -.105c.01 -.04 .022 -.077 .035 -.112l.046 -.105l.059 -.1l4 -6a1 1 0 0 1 1.165 -.39l.114 .05l3.277 1.638l3.495 -4.369z"
        stroke - width = "0" fill = "currentColor" /> < title > { title } < / title > < /
        svg >
    }
}
