#[cfg(feature = "TbCurrencyDirham")]
use leptos::*;
#[cfg(feature = "TbCurrencyDirham")]
///This icon requires the feature `TbCurrencyDirham` to be enabled.
#[component]
pub fn CurrencyDirham(
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
        "icon icon-tabler icon-tabler-currency-dirham" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8.5 19h-3.5" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8.599 16.479a1.5 1.5 0 1 0 -1.099 2.521" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7 4v9" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M15 13h1.888a1.5 1.5 0 0 0 1.296 -2.256l-2.184 -3.744" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M11 13.01v-.01" /> < title > { title } < /
        title > < / svg >
    }
}
