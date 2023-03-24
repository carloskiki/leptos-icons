#[cfg(feature = "TbSpray")]
use leptos::*;
#[cfg(feature = "TbSpray")]
///This icon requires the feature `TbSpray` to be enabled.
#[component]
pub fn Spray(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-spray"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M4 10m0 2a2 2 0 0 1 2 -2h4a2 2 0 0 1 2 2v7a2 2 0 0 1 -2 2h-4a2 2 0 0 1 -2 -2z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6 10v-4a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 7h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 9h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 5h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M21 3h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M21 7h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M21 11h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 7h1" /> < title > { title } < / title > < /
        svg >
    }
}
