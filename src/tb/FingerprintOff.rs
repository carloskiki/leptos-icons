#[cfg(feature = "TbFingerprintOff")]
use leptos::*;
#[cfg(feature = "TbFingerprintOff")]
///This icon requires the feature `TbFingerprintOff` to be enabled.
#[component]
pub fn FingerprintOff(
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
        "icon icon-tabler icon-tabler-fingerprint-off" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M18.9 7a8 8 0 0 1 1.1 5v1a6 6 0 0 0 .8 3" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M8 11c0 -.848 .264 -1.634 .713 -2.28m2.4 -1.621a4 4 0 0 1 4.887 3.901l0 1" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M12 12v1a14 14 0 0 0 2.5 8" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M8 15a18 18 0 0 0 1.8 6" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M4.9 19a22 22 0 0 1 -.9 -7v-1a8 8 0 0 1 1.854 -5.143m2.176 -1.825a8 8 0 0 1 7.97 .018"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > {
        title } < / title > < / svg >
    }
}
