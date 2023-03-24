#[cfg(feature = "TbWallOff")]
use leptos::*;
#[cfg(feature = "TbWallOff")]
///This icon requires the feature `TbWallOff` to be enabled.
#[component]
pub fn WallOff(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-wall-off"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 4h10a2 2 0 0 1 2 2v10m-.589 3.417c-.361 .36 -.86 .583 -1.411 .583h-12a2 2 0 0 1 -2 -2v-12c0 -.55 .222 -1.047 .58 -1.409"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M4 8h4m4 0h8" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 12h-4m-4 0h-8" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 16h12" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 4v1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 8v2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 12v4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M11 16v4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
