#[cfg(feature = "FiMicOff")]
use leptos::*;
#[cfg(feature = "FiMicOff")]
///This icon requires the feature `FiMicOff` to be enabled.
#[component]
pub fn MicOff(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap =
        "round" stroke - linejoin = "round" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < line xmlns = "http://www.w3.org/2000/svg" x1 =
        "1" y1 = "1" x2 = "23" y2 = "23" />< path xmlns = "http://www.w3.org/2000/svg" d
        = "M9 9v3a3 3 0 0 0 5.12 2.12M15 9.34V4a3 3 0 0 0-5.94-.6" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M17 16.95A7 7 0 0 1 5 12v-2m14 0v2a7 7 0 0 1-.11 1.23" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "12" y1 = "19" x2 = "12" y2 = "23" />< line
        xmlns = "http://www.w3.org/2000/svg" x1 = "8" y1 = "23" x2 = "16" y2 = "23" /> <
        title > { title } < / title > < / svg >
    }
}
