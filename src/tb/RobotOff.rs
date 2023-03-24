#[cfg(feature = "TbRobotOff")]
use leptos::*;
#[cfg(feature = "TbRobotOff")]
///This icon requires the feature `TbRobotOff` to be enabled.
#[component]
pub fn RobotOff(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-robot-off"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M11 7h6a2 2 0 0 1 2 2v1l1 1v3l-1 1m-.171 3.811a2 2 0 0 1 -1.829 1.189h-10a2 2 0 0 1 -2 -2v-3l-1 -1v-3l1 -1v-1a2 2 0 0 1 2 -2"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M10 16h4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8.5 11.5m-1 0a1 1 0 1 0 2 0a1 1 0 1 0 -2 0"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15.854 11.853a.498 .498 0 0 0 -.354 -.853a.498 .498 0 0 0 -.356 .149" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M8.336 4.343l-.336 -1.343" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M15 7l1 -4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
