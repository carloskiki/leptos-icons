#[cfg(feature = "TbCreditCardOff")]
use leptos::*;
#[cfg(feature = "TbCreditCardOff")]
///This icon requires the feature `TbCreditCardOff` to be enabled.
#[component]
pub fn CreditCardOff(
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
        "icon icon-tabler icon-tabler-credit-card-off" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M3 3l18 18" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 5h9a3 3 0 0 1 3 3v8a3 3 0 0 1 -.128 .87" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M18.87 18.872a3 3 0 0 1 -.87 .128h-12a3 3 0 0 1 -3 -3v-8c0 -1.352 .894 -2.495 2.124 -2.87"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M3 11l8 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 11l6 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7 15l.01 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M11 15l2 0" /> < title > { title } < / title >
        < / svg >
    }
}
