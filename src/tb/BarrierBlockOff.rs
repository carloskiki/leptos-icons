#[cfg(feature = "TbBarrierBlockOff")]
use leptos::*;
#[cfg(feature = "TbBarrierBlockOff")]
///This icon requires the feature `TbBarrierBlockOff` to be enabled.
#[component]
pub fn BarrierBlockOff(
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
        "icon icon-tabler icon-tabler-barrier-block-off" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d
        = "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M11 7h8a1 1 0 0 1 1 1v7c0 .27 -.107 .516 -.282 .696" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 16h-11a1 1 0 0 1 -1 -1v-7a1 1 0 0 1 1 -1h2"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M7 16v4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7.5 16l4.244 -4.244" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M13.745 9.755l2.755 -2.755" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M13.5 16l1.249 -1.249" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16.741 12.759l3.259 -3.259" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 13.5l4.752 -4.752" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 17v3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5 20h4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 20h4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 7v-2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
