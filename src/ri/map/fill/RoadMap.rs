#[cfg(feature = "RiMapFillRoadMap")]
use leptos::*;
#[cfg(feature = "RiMapFillRoadMap")]
///This icon requires the feature `RiMapFillRoadMap` to be enabled.
#[component]
pub fn RoadMap(
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
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M16.95 11.95a6.996 6.996 0 0 0 1.858-6.582l2.495-1.07a.5.5 0 0 1 .697.46V19l-7 3-6-3-6.303 2.701a.5.5 0 0 1-.697-.46V7l3.129-1.341a6.993 6.993 0 0 0 1.921 6.29L12 16.9l4.95-4.95zm-1.414-1.414L12 14.07l-3.536-3.535a5 5 0 1 1 7.072 0z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
