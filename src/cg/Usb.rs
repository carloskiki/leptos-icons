#[cfg(feature = "CgUsb")]
use leptos::*;
#[cfg(feature = "CgUsb")]
///This icon requires the feature `CgUsb` to be enabled.
#[component]
pub fn Usb(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M10 4.5H11V6.5H10V4.5Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 4.5H13V6.5H14V4.5Z" fill = "currentColor"
        />< path xmlns = "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule
        = "evenodd" d =
        "M7 8.5V1.5H17V8.5H19V19.5C19 21.1569 17.6569 22.5 16 22.5H8C6.34315 22.5 5 21.1569 5 19.5V8.5H7ZM9 3.5H15V8.5H9V3.5ZM17 10.5H7V19.5C7 20.0523 7.44772 20.5 8 20.5H16C16.5523 20.5 17 20.0523 17 19.5V10.5Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
