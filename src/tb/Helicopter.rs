#[cfg(feature = "TbHelicopter")]
use leptos::*;
#[cfg(feature = "TbHelicopter")]
///This icon requires the feature `TbHelicopter` to be enabled.
#[component]
pub fn Helicopter(
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
        "icon icon-tabler icon-tabler-helicopter" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3 10l1 2h6" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 9a2 2 0 0 0 -2 2v3c0 1.1 .9 2 2 2h7a2 2 0 0 0 2 -2c0 -3.31 -3.13 -5 -7 -5h-2z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M13 9l0 -3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5 6l15 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 9.1v3.9h5.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 19l0 -3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19 19l-8 0" /> < title > { title } < / title >
        < / svg >
    }
}
