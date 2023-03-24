#[cfg(feature = "TbChartCandleFilled")]
use leptos::*;
#[cfg(feature = "TbChartCandleFilled")]
///This icon requires the feature `TbChartCandleFilled` to be enabled.
#[component]
pub fn ChartCandleFilled(
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
        "icon icon-tabler icon-tabler-chart-candle-filled" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6 3a1 1 0 0 1 .993 .883l.007 .117v1a2 2 0 0 1 1.995 1.85l.005 .15v3a2 2 0 0 1 -1.85 1.995l-.15 .005v8a1 1 0 0 1 -1.993 .117l-.007 -.117v-8a2 2 0 0 1 -1.995 -1.85l-.005 -.15v-3a2 2 0 0 1 1.85 -1.995l.15 -.005v-1a1 1 0 0 1 1 -1z"
        stroke - width = "0" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12 3a1 1 0 0 1 .993 .883l.007 .117v9a2 2 0 0 1 1.995 1.85l.005 .15v3a2 2 0 0 1 -1.85 1.995l-.15 .005a1 1 0 0 1 -1.993 .117l-.007 -.117l-.15 -.005a2 2 0 0 1 -1.844 -1.838l-.006 -.157v-3a2 2 0 0 1 1.85 -1.995l.15 -.005v-9a1 1 0 0 1 1 -1z"
        stroke - width = "0" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M18 3a1 1 0 0 1 .993 .883l.007 .117a2 2 0 0 1 1.995 1.85l.005 .15v4a2 2 0 0 1 -1.85 1.995l-.15 .005v8a1 1 0 0 1 -1.993 .117l-.007 -.117v-8a2 2 0 0 1 -1.995 -1.85l-.005 -.15v-4a2 2 0 0 1 1.85 -1.995l.15 -.005a1 1 0 0 1 1 -1z"
        stroke - width = "0" fill = "currentColor" /> < title > { title } < / title > < /
        svg >
    }
}
