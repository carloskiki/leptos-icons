#[cfg(feature = "TbCpuOff")]
use leptos::*;
#[cfg(feature = "TbCpuOff")]
///This icon requires the feature `TbCpuOff` to be enabled.
#[component]
pub fn CpuOff(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-cpu-off"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9 5h9a1 1 0 0 1 1 1v9m-.292 3.706a1 1 0 0 1 -.708 .294h-12a1 1 0 0 1 -1 -1v-12c0 -.272 .108 -.518 .284 -.698"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M13 9h2v2m0 4h-6v-6" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M3 10h2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 14h2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 3v2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 3v2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M21 10h-2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M21 14h-2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 21v-2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 21v-2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
