#[cfg(feature = "RiLogosLineSafari")]
use leptos::*;
#[cfg(feature = "RiLogosLineSafari")]
///This icon requires the feature `RiLogosLineSafari` to be enabled.
#[component]
pub fn Safari(
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
        "M17.812 6.503l-4.398 6.911-6.911 4.398A7.973 7.973 0 0 0 11 19.938V18h2v1.938a7.96 7.96 0 0 0 3.906-1.618l-1.37-1.37 1.414-1.414 1.37 1.37A7.96 7.96 0 0 0 19.938 13H18v-2h1.938a7.973 7.973 0 0 0-2.126-4.497zm-.315-.315A7.973 7.973 0 0 0 13 4.062V6h-2V4.062A7.96 7.96 0 0 0 7.094 5.68l1.37 1.37L7.05 8.464l-1.37-1.37A7.96 7.96 0 0 0 4.062 11H6v2H4.062a7.973 7.973 0 0 0 2.126 4.497l4.398-6.911 6.911-4.398zM12 22C6.477 22 2 17.523 2 12S6.477 2 12 2s10 4.477 10 10-4.477 10-10 10z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
