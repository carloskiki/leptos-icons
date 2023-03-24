#[cfg(feature = "TbCupOff")]
use leptos::*;
#[cfg(feature = "TbCupOff")]
///This icon requires the feature `TbCupOff` to be enabled.
#[component]
pub fn CupOff(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-cup-off"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 8h-3v3h6m4 0h4v-3h-7" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M17.5 11l-.323 2.154m-.525 3.497l-.652 4.349h-8l-1.5 -10" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M6 8v-1c0 -.296 .064 -.577 .18 -.83m2.82 -1.17h7a2 2 0 0 1 2 2v1" />< path xmlns
        = "http://www.w3.org/2000/svg" d = "M15 5v-2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
