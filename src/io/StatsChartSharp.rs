#[cfg(feature = "IoStatsChartSharp")]
use leptos::*;
#[cfg(feature = "IoStatsChartSharp")]
///This icon requires the feature `IoStatsChartSharp` to be enabled.
#[component]
pub fn StatsChartSharp(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M128,496H48V304h80Z" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M352,496H272V208h80Z" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M464,496H384V96h80Z" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M240,496H160V16h80Z" /> < title > { title } < / title > < / svg >
    }
}
