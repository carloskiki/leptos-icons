#[cfg(feature = "FiRepeat")]
use leptos::*;
#[cfg(feature = "FiRepeat")]
///This icon requires the feature `FiRepeat` to be enabled.
#[component]
pub fn Repeat(
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
        "http://www.w3.org/2000/svg" > < polyline xmlns = "http://www.w3.org/2000/svg"
        points = "17 1 21 5 17 9" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3 11V9a4 4 0 0 1 4-4h14" />< polyline xmlns = "http://www.w3.org/2000/svg"
        points = "7 23 3 19 7 15" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M21 13v2a4 4 0 0 1-4 4H3" /> < title > { title } < / title > < / svg >
    }
}
