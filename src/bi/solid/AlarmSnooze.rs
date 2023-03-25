#[cfg(feature = "BiSolidAlarmSnooze")]
use leptos::*;
#[cfg(feature = "BiSolidAlarmSnooze")]
///This icon requires the feature `BiSolidAlarmSnooze` to be enabled.
#[component]
pub fn AlarmSnooze(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "m17.284 3.707 1.412-1.416 3.01 3-1.413 1.417zm-10.586 0-2.99 2.999L2.29 5.294l2.99-3zM12 4c-4.878 0-9 4.121-9 9s4.122 9 9 9c4.879 0 9-4.121 9-9s-4.121-9-9-9zm4 13H8.131l4-6H8V9h7.868l-1.035 1.554-.001.001L11.869 15H16v2z"
        /> < title > { title } < / title > < / svg >
    }
}
