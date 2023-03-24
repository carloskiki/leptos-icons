#[cfg(feature = "RiDeviceFillSignalWifi1")]
use leptos::*;
#[cfg(feature = "RiDeviceFillSignalWifi1")]
///This icon requires the feature `RiDeviceFillSignalWifi1` to be enabled.
#[component]
pub fn SignalWifi1(
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
        "M12 3c4.284 0 8.22 1.497 11.31 3.996L12 21 .69 6.997C3.78 4.497 7.714 3 12 3zm0 2c-3.028 0-5.923.842-8.42 2.392l5.108 6.324C9.698 13.256 10.818 13 12 13c1.181 0 2.303.256 3.312.716L20.42 7.39C17.922 5.841 15.027 5 12 5z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
