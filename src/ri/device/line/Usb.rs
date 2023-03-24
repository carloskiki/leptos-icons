#[cfg(feature = "RiDeviceLineUsb")]
use leptos::*;
#[cfg(feature = "RiDeviceLineUsb")]
///This icon requires the feature `RiDeviceLineUsb` to be enabled.
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0H24V24H0z" />< path d
        =
        "M12 1l3 5h-2v7.381l3-1.499-.001-.882H15V7h4v4h-1.001L18 13.118l-5 2.5v1.553c1.166.412 2 1.523 2 2.829 0 1.657-1.343 3-3 3s-3-1.343-3-3c0-1.187.69-2.213 1.69-2.7L6 14l-.001-2.268C5.402 11.386 5 10.74 5 10c0-1.105.895-2 2-2s2 .895 2 2c0 .74-.402 1.387-1 1.732V13l3 2.086V6H9l3-5zm0 18c-.552 0-1 .448-1 1s.448 1 1 1 1-.448 1-1-.448-1-1-1z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
