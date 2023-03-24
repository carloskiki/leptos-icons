#[cfg(feature = "FiWifiOff")]
use leptos::*;
#[cfg(feature = "FiWifiOff")]
///This icon requires the feature `FiWifiOff` to be enabled.
#[component]
pub fn WifiOff(
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
        fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap =
        "round" stroke - linejoin = "round" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < line xmlns = "http://www.w3.org/2000/svg" x1 =
        "1" y1 = "1" x2 = "23" y2 = "23" />< path xmlns = "http://www.w3.org/2000/svg" d
        = "M16.72 11.06A10.94 10.94 0 0 1 19 12.55" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5 12.55a10.94 10.94 0 0 1 5.17-2.39" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M10.71 5.05A16 16 0 0 1 22.58 9" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M1.42 9a15.91 15.91 0 0 1 4.7-2.88" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8.53 16.11a6 6 0 0 1 6.95 0" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "12" y1 = "20" x2 = "12.01" y2 = "20" /> <
        title > { title } < / title > < / svg >
    }
}
