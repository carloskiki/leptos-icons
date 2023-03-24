#[cfg(feature = "TbLifebuoyOff")]
use leptos::*;
#[cfg(feature = "TbLifebuoyOff")]
///This icon requires the feature `TbLifebuoyOff` to be enabled.
#[component]
pub fn LifebuoyOff(
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
        "icon icon-tabler icon-tabler-lifebuoy-off" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9.171 9.172a4 4 0 0 0 5.65 5.663m1.179 -2.835a4 4 0 0 0 -4 -4" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M5.64 5.632a9 9 0 1 0 12.73 12.725m1.667 -2.301a9 9 0 0 0 -12.077 -12.1" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M15 15l3.35 3.35" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 15l-3.35 3.35" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5.65 5.65l3.35 3.35" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18.35 5.65l-3.35 3.35" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
