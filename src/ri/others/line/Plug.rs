#[cfg(feature = "RiOthersLinePlug")]
use leptos::*;
#[cfg(feature = "RiOthersLinePlug")]
///This icon requires the feature `RiOthersLinePlug` to be enabled.
#[component]
pub fn Plug(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path
        fill - rule = "nonzero" d =
        "M13 18v2h6v2h-6a2 2 0 0 1-2-2v-2H8a4 4 0 0 1-4-4V7a1 1 0 0 1 1-1h3V2h2v4h4V2h2v4h3a1 1 0 0 1 1 1v7a4 4 0 0 1-4 4h-3zm-5-2h8a2 2 0 0 0 2-2v-3H6v3a2 2 0 0 0 2 2zm10-8H6v1h12V8zm-6 6.5a1 1 0 1 1 0-2 1 1 0 0 1 0 2z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
