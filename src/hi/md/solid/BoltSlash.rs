#[cfg(feature = "HiMdSolidBoltSlash")]
use leptos::*;
#[cfg(feature = "HiMdSolidBoltSlash")]
///This icon requires the feature `HiMdSolidBoltSlash` to be enabled.
#[component]
pub fn BoltSlash(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M2.21967 2.21967C2.51256 1.92678 2.98744 1.92678 3.28033 2.21967L17.7803 16.7197C18.0732 17.0126 18.0732 17.4874 17.7803 17.7803C17.4874 18.0732 17.0126 18.0732 16.7197 17.7803L2.21967 3.28033C1.92678 2.98744 1.92678 2.51256 2.21967 2.21967Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M4.73016 7.91214L2.19108 10.7499C1.99385 10.9704 1.9446 11.2861 2.06533 11.5562C2.18607 11.8262 2.45423 12 2.75001 12H8.81805L4.73016 7.91214Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9.23329 12.4153L8.01666 18.0929C7.9454 18.4255 8.10685 18.7644 8.41002 18.9185C8.71318 19.0727 9.08215 19.0036 9.30894 18.7501L12.2647 15.4467L9.23329 12.4153Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15.2699 12.0879L17.8089 9.25013C18.0062 9.0297 18.0554 8.71393 17.9347 8.4439C17.814 8.17388 17.5458 8.00003 17.25 8.00003H11.182L15.2699 12.0879Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10.7667 7.58476L11.9834 1.90718C12.0546 1.57461 11.8932 1.23571 11.59 1.08152C11.2868 0.927338 10.9179 0.996463 10.6911 1.24994L7.73537 4.55339L10.7667 7.58476Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
