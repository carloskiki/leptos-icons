#[cfg(feature = "TbBoxAlignRight")]
use leptos::*;
#[cfg(feature = "TbBoxAlignRight")]
///This icon requires the feature `TbBoxAlignRight` to be enabled.
#[component]
pub fn BoxAlignRight(
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
        "icon icon-tabler icon-tabler-box-align-right" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M14.248 19.753v-16h5a1 1 0 0 1 1 1v14a1 1 0 0 1 -1 1h-5z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9.248 19.753h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4.247 19.753h.011" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4.247 14.752h.011" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4.247 8.752h.011" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4.247 3.752h.011" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9.248 3.752h.01" /> < title > { title } < /
        title > < / svg >
    }
}
