#[cfg(feature = "IoPieChartSharp")]
use leptos::*;
#[cfg(feature = "IoPieChartSharp")]
///This icon requires the feature `IoPieChartSharp` to be enabled.
#[component]
pub fn PieChartSharp(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M58,362.09,51.49,347.5A224,224,0,0,1,256,32h16V266.37Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M304,66.46V287.11L94.62,380.78A208.31,208.31,0,0,0,272,480c114.69,0,208-93.31,208-208C480,168.19,403.55,81.9,304,66.46Z"
        /> < title > { title } < / title > < / svg >
    }
}
