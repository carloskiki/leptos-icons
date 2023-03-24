#[cfg(feature = "TbAbacusOff")]
use leptos::*;
#[cfg(feature = "TbAbacusOff")]
///This icon requires the feature `TbAbacusOff` to be enabled.
#[component]
pub fn AbacusOff(
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
        "icon icon-tabler icon-tabler-abacus-off" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5 5v16" />< path xmlns = "http://www.w3.org/2000/svg" d = "M19 21v-2m0 -4v-12"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M5 7h2m4 0h8" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5 15h10" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 13v4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M11 13v4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 16v1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 5v4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M11 5v2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 8v1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 21h18" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
