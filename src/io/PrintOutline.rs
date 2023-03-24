#[cfg(feature = "IoPrintOutline")]
use leptos::*;
#[cfg(feature = "IoPrintOutline")]
///This icon requires the feature `IoPrintOutline` to be enabled.
#[component]
pub fn PrintOutline(
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
        "M384,368h24a40.12,40.12,0,0,0,40-40V168a40.12,40.12,0,0,0-40-40H104a40.12,40.12,0,0,0-40,40V328a40.12,40.12,0,0,0,40,40h24"
        style = "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" />< rect
        xmlns = "http://www.w3.org/2000/svg" x = "128" y = "240" width = "256" height =
        "208" rx = "24.32" ry = "24.32" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M384,128V104a40.12,40.12,0,0,0-40-40H168a40.12,40.12,0,0,0-40,40v24" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" />< circle xmlns
        = "http://www.w3.org/2000/svg" cx = "392" cy = "184" r = "24" /> < title > {
        title } < / title > < / svg >
    }
}
