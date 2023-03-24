#[cfg(feature = "TbBiohazardOff")]
use leptos::*;
#[cfg(feature = "TbBiohazardOff")]
///This icon requires the feature `TbBiohazardOff` to be enabled.
#[component]
pub fn BiohazardOff(
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
        "icon icon-tabler icon-tabler-biohazard-off" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10.586 10.586a2 2 0 1 0 2.836 2.82" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M11.939 14c0 .173 .048 .351 .056 .533v.217a4.75 4.75 0 0 1 -4.533 4.745h-.217"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M2.495 14.745a4.75 4.75 0 0 1 7.737 -3.693" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M16.745 19.495a4.75 4.75 0 0 1 -4.69 -5.503h-.06" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M14.533 10.538a4.75 4.75 0 0 1 6.957 3.987v.217" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M10.295 10.929a4.75 4.75 0 0 1 -2.988 -3.64m.66 -3.324a4.75 4.75 0 0 1 .5 -.66l.164 -.172"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15.349 3.133a4.75 4.75 0 0 1 -.836 7.385" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
