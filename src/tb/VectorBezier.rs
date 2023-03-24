#[cfg(feature = "TbVectorBezier")]
use leptos::*;
#[cfg(feature = "TbVectorBezier")]
///This icon requires the feature `TbVectorBezier` to be enabled.
#[component]
pub fn VectorBezier(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-vector-bezier" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3 14m0 1a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M17 14m0 1a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10 6m0 1a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M10 8.5a6 6 0 0 0 -5 5.5" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M14 8.5a6 6 0 0 1 5 5.5" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M10 8l-6 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 8l-6 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 8m-1 0a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M21 8m-1 0a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> < title > { title } < / title > < /
        svg >
    }
}
