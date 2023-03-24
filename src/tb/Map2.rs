#[cfg(feature = "TbMap2")]
use leptos::*;
#[cfg(feature = "TbMap2")]
///This icon requires the feature `TbMap2` to be enabled.
#[component]
pub fn Map2(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-map-2"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d = "M18 6l0 .01" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M18 13l-3.5 -5a4 4 0 1 1 7 0l-3.5 5" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M10.5 4.75l-1.5 -.75l-6 3l0 13l6 -3l6 3l6 -3l0 -2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 4l0 13" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 15l0 5" /> < title > { title } < / title >
        < / svg >
    }
}
