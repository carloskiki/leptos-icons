#[cfg(feature = "ImStopwatch")]
use leptos::*;
#[cfg(feature = "ImStopwatch")]
///This icon requires the feature `ImStopwatch` to be enabled.
#[component]
pub fn Stopwatch(
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
        "M8 3.019v-1.019h2v-1c0-0.552-0.448-1-1-1h-3c-0.552 0-1 0.448-1 1v1h2v1.019c-3.356 0.255-6 3.059-6 6.481 0 3.59 2.91 6.5 6.5 6.5s6.5-2.91 6.5-6.5c0-3.422-2.644-6.226-6-6.481zM11.036 13.036c-0.944 0.944-2.2 1.464-3.536 1.464s-2.591-0.52-3.536-1.464c-0.944-0.944-1.464-2.2-1.464-3.536s0.52-2.591 1.464-3.536c0.907-0.907 2.101-1.422 3.377-1.462l-0.339 4.907c-0.029 0.411 0.195 0.591 0.497 0.591s0.527-0.18 0.497-0.591l-0.339-4.907c1.276 0.040 2.47 0.555 3.377 1.462 0.944 0.944 1.464 2.2 1.464 3.536s-0.52 2.591-1.464 3.536z"
        /> < title > { title } < / title > < / svg >
    }
}
