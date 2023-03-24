#[cfg(feature = "TbTopologyComplex")]
use leptos::*;
#[cfg(feature = "TbTopologyComplex")]
///This icon requires the feature `TbTopologyComplex` to be enabled.
#[component]
pub fn TopologyComplex(
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
        "icon icon-tabler icon-tabler-topology-complex" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d
        = "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M20 18a2 2 0 1 0 -4 0a2 2 0 0 0 4 0z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 18a2 2 0 1 0 -4 0a2 2 0 0 0 4 0z" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M8 6a2 2 0 1 0 -4 0a2 2 0 0 0 4 0z" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M20 6a2 2 0 1 0 -4 0a2 2 0 0 0 4 0z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 12a2 2 0 1 0 -4 0a2 2 0 0 0 4 0z" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M7.5 7.5l3 3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M6 8v8" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 16v-8" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 6h8" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 18h-8" /> < title > { title } < / title > <
        / svg >
    }
}
