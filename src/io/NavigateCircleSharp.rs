#[cfg(feature = "IoNavigateCircleSharp")]
use leptos::*;
#[cfg(feature = "IoNavigateCircleSharp")]
///This icon requires the feature `IoNavigateCircleSharp` to be enabled.
#[component]
pub fn NavigateCircleSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M256,48h0A208.23,208.23,0,0,0,48,256c0,114.68,93.31,208,208,208h0A208.23,208.23,0,0,0,464,256C464,141.31,370.69,48,256,48Zm-8,361V264H104l-1,0,259-114.11Z"
        /> < title > { title } < / title > < / svg >
    }
}
