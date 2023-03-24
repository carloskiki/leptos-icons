#[cfg(feature = "ImAngry2")]
use leptos::*;
#[cfg(feature = "ImAngry2")]
///This icon requires the feature `ImAngry2` to be enabled.
#[component]
pub fn Angry2(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M8 0c-4.418 0-8 3.582-8 8s3.582 8 8 8 8-3.582 8-8-3.582-8-8-8zM9.001 5.946c0.032-0.741 0.706-1.234 1.275-1.518 0.543-0.271 1.080-0.407 1.102-0.413 0.268-0.067 0.539 0.096 0.606 0.364s-0.096 0.539-0.364 0.606c-0.275 0.070-0.602 0.189-0.89 0.334 0.166 0.179 0.268 0.418 0.268 0.681 0 0.552-0.448 1-1 1s-1-0.448-1-1c0-0.018 0.001-0.036 0.002-0.054zM4.015 4.379c0.067-0.268 0.338-0.431 0.606-0.364 0.023 0.006 0.559 0.141 1.102 0.413 0.568 0.284 1.243 0.776 1.275 1.518 0.001 0.018 0.002 0.036 0.002 0.054 0 0.552-0.448 1-1 1s-1-0.448-1-1c0-0.263 0.102-0.503 0.268-0.681-0.288-0.144-0.614-0.264-0.89-0.334-0.268-0.067-0.431-0.338-0.364-0.606zM11.002 12.199c-0.612-1.018-1.727-1.699-3.002-1.699s-2.389 0.681-3.002 1.699l-1.286-0.772c0.874-1.454 2.467-2.427 4.288-2.427s3.414 0.973 4.288 2.427l-1.286 0.772z"
        /> < title > { title } < / title > < / svg >
    }
}
