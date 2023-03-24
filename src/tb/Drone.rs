#[cfg(feature = "TbDrone")]
use leptos::*;
#[cfg(feature = "TbDrone")]
///This icon requires the feature `TbDrone` to be enabled.
#[component]
pub fn Drone(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-drone"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M10 10h4v4h-4z" />< path xmlns
        = "http://www.w3.org/2000/svg" d = "M10 10l-3.5 -3.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9.96 6a3.5 3.5 0 1 0 -3.96 3.96" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M14 10l3.5 -3.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 9.96a3.5 3.5 0 1 0 -3.96 -3.96" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M14 14l3.5 3.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14.04 18a3.5 3.5 0 1 0 3.96 -3.96" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M10 14l-3.5 3.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M6 14.04a3.5 3.5 0 1 0 3.96 3.96" /> < title >
        { title } < / title > < / svg >
    }
}
