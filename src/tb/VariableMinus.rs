#[cfg(feature = "TbVariableMinus")]
use leptos::*;
#[cfg(feature = "TbVariableMinus")]
///This icon requires the feature `TbVariableMinus` to be enabled.
#[component]
pub fn VariableMinus(
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
        "icon icon-tabler icon-tabler-variable-minus" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 16c1.5 0 3 -2 4 -3.5s2.5 -3.5 4 -3.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M5 4c-2.5 5 -2.5 10 0 16m14 -16c1.775 3.55 2.29 7.102 1.544 11.01m-11.544 -6.01h1c1 0 1 1 2.016 3.527c.782 1.966 .943 3 1.478 3.343"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 16c1.5 0 3 -2 4 -3.5s2.5 -3.5 4 -3.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 19h6" /> < title > { title } < / title > <
        / svg >
    }
}
