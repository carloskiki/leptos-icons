#[cfg(feature = "CgUsbC")]
use leptos::*;
#[cfg(feature = "CgUsbC")]
///This icon requires the feature `CgUsbC` to be enabled.
#[component]
pub fn UsbC(
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
        "M8 11C7.44772 11 7 11.4477 7 12C7 12.5523 7.44772 13 8 13H16C16.5523 13 17 12.5523 17 12C17 11.4477 16.5523 11 16 11H8Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M3 12C3 9.23858 5.23858 7 8 7H16C18.7614 7 21 9.23858 21 12C21 14.7614 18.7614 17 16 17H8C5.23858 17 3 14.7614 3 12ZM8 9H16C17.6569 9 19 10.3431 19 12C19 13.6569 17.6569 15 16 15H8C6.34315 15 5 13.6569 5 12C5 10.3431 6.34315 9 8 9Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
