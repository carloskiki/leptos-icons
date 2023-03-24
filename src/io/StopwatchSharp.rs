#[cfg(feature = "IoStopwatchSharp")]
use leptos::*;
#[cfg(feature = "IoStopwatchSharp")]
///This icon requires the feature `IoStopwatchSharp` to be enabled.
#[component]
pub fn StopwatchSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M280,81.5V48H232V81.5a191,191,0,0,0-84.43,32.13L120,86,86,120l25.59,25.59A191.17,191.17,0,0,0,64,272c0,105.87,86.13,192,192,192s192-86.13,192-192C448,174.26,374.58,93.34,280,81.5ZM256,320a48,48,0,0,1-16-93.25V136h32v90.75A48,48,0,0,1,256,320Z"
        /> < title > { title } < / title > < / svg >
    }
}
