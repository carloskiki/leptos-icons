#[cfg(feature = "TiCalculator")]
use leptos::*;
#[cfg(feature = "TiCalculator")]
///This icon requires the feature `TiCalculator` to be enabled.
#[component]
pub fn Calculator(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M17 21h-8c-1.7 0-3-1.3-3-3v-12c0-1.7 1.3-3 3-3h8c1.7 0 3 1.3 3 3v12c0 1.7-1.3 3-3 3zm-8-16c-.6 0-1 .4-1 1v12c0 .6.4 1 1 1h8c.6 0 1-.4 1-1v-12c0-.6-.4-1-1-1h-8z"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "10" cy = "11" r = "1" /><
        circle xmlns = "http://www.w3.org/2000/svg" cx = "13" cy = "11" r = "1" /><
        circle xmlns = "http://www.w3.org/2000/svg" cx = "16" cy = "11" r = "1" /><
        circle xmlns = "http://www.w3.org/2000/svg" cx = "10" cy = "14" r = "1" /><
        circle xmlns = "http://www.w3.org/2000/svg" cx = "13" cy = "14" r = "1" /><
        circle xmlns = "http://www.w3.org/2000/svg" cx = "16" cy = "14" r = "1" /><
        circle xmlns = "http://www.w3.org/2000/svg" cx = "10" cy = "17" r = "1" /><
        circle xmlns = "http://www.w3.org/2000/svg" cx = "13" cy = "17" r = "1" /><
        circle xmlns = "http://www.w3.org/2000/svg" cx = "16" cy = "17" r = "1" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M16 7v1h-6v-1h6m1-1h-8v3h8v-3z" /> <
        title > { title } < / title > < / svg >
    }
}
