#[cfg(feature = "TbZoomInArea")]
use leptos::*;
#[cfg(feature = "TbZoomInArea")]
///This icon requires the feature `TbZoomInArea` to be enabled.
#[component]
pub fn ZoomInArea(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-zoom-in-area" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15 13v4" />< path xmlns = "http://www.w3.org/2000/svg" d = "M13 15h4" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M15 15m-5 0a5 5 0 1 0 10 0a5 5 0 1 0 -10 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M22 22l-3 -3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M6 18h-1a2 2 0 0 1 -2 -2v-1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 11v-1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 6v-1a2 2 0 0 1 2 -2h1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 3h1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 3h1a2 2 0 0 1 2 2v1" /> < title > { title }
        < / title > < / svg >
    }
}
