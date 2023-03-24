#[cfg(feature = "RiWeatherLineTempHot")]
use leptos::*;
#[cfg(feature = "RiWeatherLineTempHot")]
///This icon requires the feature `RiWeatherLineTempHot` to be enabled.
#[component]
pub fn TempHot(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path
        fill - rule = "nonzero" d =
        "M8 5a4 4 0 1 1 8 0v5.255a7 7 0 1 1-8 0V5zm1.144 6.895a5 5 0 1 0 5.712 0L14 11.298V5a2 2 0 1 0-4 0v6.298l-.856.597zm1.856.231V5h2v7.126A4.002 4.002 0 0 1 12 20a4 4 0 0 1-1-7.874zM12 18a2 2 0 1 0 0-4 2 2 0 0 0 0 4z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
