#[cfg(feature = "TbMathXPlusX")]
use leptos::*;
#[cfg(feature = "TbMathXPlusX")]
///This icon requires the feature `TbMathXPlusX` to be enabled.
#[component]
pub fn MathXPlusX(
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
        "icon icon-tabler icon-tabler-math-x-plus-x" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M2 9l6 6" />< path xmlns = "http://www.w3.org/2000/svg" d = "M2 15l6 -6" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M16 9l6 6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 15l6 -6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 12h4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 10v4" /> < title > { title } < / title > <
        / svg >
    }
}
