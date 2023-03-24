#[cfg(feature = "TbBarbellOff")]
use leptos::*;
#[cfg(feature = "TbBarbellOff")]
///This icon requires the feature `TbBarbellOff` to be enabled.
#[component]
pub fn BarbellOff(
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
        "icon icon-tabler icon-tabler-barbell-off" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M2 12h1" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6 8h-2a1 1 0 0 0 -1 1v6a1 1 0 0 0 1 1h2" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M6.298 6.288a1 1 0 0 0 -.298 .712v10a1 1 0 0 0 1 1h1a1 1 0 0 0 1 -1v-8" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M9 12h3" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M15 15v2a1 1 0 0 0 1 1h1c.275 0 .523 -.11 .704 -.29m.296 -3.71v-7a1 1 0 0 0 -1 -1h-1a1 1 0 0 0 -1 1v4"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M18 8h2a1 1 0 0 1 1 1v6a1 1 0 0 1 -1 1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M22 12h-1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
