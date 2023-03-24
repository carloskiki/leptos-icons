#[cfg(feature = "IoEyedropOutline")]
use leptos::*;
#[cfg(feature = "IoEyedropOutline")]
///This icon requires the feature `IoEyedropOutline` to be enabled.
#[component]
pub fn EyedropOutline(
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
        "M262.51,204.22,70,396.69C57.56,409.15,48,464,48,464s54.38-9.09,67.31-22L307.8,249.51"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< rect xmlns = "http://www.w3.org/2000/svg" x = "211.72" y = "172.19" width =
        "192.15" height = "64.05" rx = "32.03" ry = "32.03" transform =
        "translate(234.54 -157.83) rotate(45)" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M289.91,141s20.57,8.57,37.22-8.08L381.8,62.29c18.5-19.41,49.26-18.69,67.94,0h0c18.68,18.68,19.34,48.81,0,67.93l-70.68,54.67c-15.65,15.65-8.08,37.22-8.08,37.22"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M115.31,442s-26.48,17.34-44.56-.73S70,396.69,70,396.69" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        /> < title > { title } < / title > < / svg >
    }
}
