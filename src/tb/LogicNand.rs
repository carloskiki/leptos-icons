#[cfg(feature = "TbLogicNand")]
use leptos::*;
#[cfg(feature = "TbLogicNand")]
///This icon requires the feature `TbLogicNand` to be enabled.
#[component]
pub fn LogicNand(
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
        "icon icon-tabler icon-tabler-logic-nand" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M22 12h-3" />< path xmlns = "http://www.w3.org/2000/svg" d = "M2 9h3" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M2 15h3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7 5c6 0 8 3.5 8 7s-2 7 -8 7h-2v-14h2z" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M17 12m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> < title > { title } < / title > < /
        svg >
    }
}
