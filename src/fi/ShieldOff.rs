#[cfg(feature = "FiShieldOff")]
use leptos::*;
#[cfg(feature = "FiShieldOff")]
///This icon requires the feature `FiShieldOff` to be enabled.
#[component]
pub fn ShieldOff(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap =
        "round" stroke - linejoin = "round" width = { size.clone() } height = { size } >
        < path xmlns = "http://www.w3.org/2000/svg" d =
        "M19.69 14a6.9 6.9 0 0 0 .31-2V5l-8-3-3.16 1.18" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M4.73 4.73L4 5v7c0 6 8 10 8 10a20.29 20.29 0 0 0 5.62-4.38" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "1" y1 = "1" x2 = "23" y2 = "23" /> < title > {
        title } < / title > < / svg >
    }
}
