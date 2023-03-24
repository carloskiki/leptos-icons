#[cfg(feature = "TiChartArea")]
use leptos::*;
#[cfg(feature = "TiChartArea")]
///This icon requires the feature `TiChartArea` to be enabled.
#[component]
pub fn ChartArea(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M20 6c0-.587-.257-1.167-.75-1.562-.863-.69-2.121-.551-2.812.312l-2.789 3.486-2.449-1.836c-.864-.648-2.087-.493-2.762.351l-4 5c-.294.368-.438.811-.438 1.249v3h16v-10zM20 19h-16c-.552 0-1 .447-1 1s.448 1 1 1h16c.552 0 1-.447 1-1s-.448-1-1-1z"
        /> < title > { title } < / title > < / svg >
    }
}
