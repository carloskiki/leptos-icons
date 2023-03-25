#[cfg(feature = "FiCoffee")]
use leptos::*;
#[cfg(feature = "FiCoffee")]
///This icon requires the feature `FiCoffee` to be enabled.
#[component]
pub fn Coffee(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = "24" height = "24"
        fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap =
        "round" stroke - linejoin = "round" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M18 8h1a4 4 0 0 1 0 8h-1" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M2 8h16v9a4 4 0 0 1-4 4H6a4 4 0 0 1-4-4V8z" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "6" y1 = "1" x2 = "6" y2 = "4" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "10" y1 = "1" x2 = "10" y2 = "4" />< line xmlns
        = "http://www.w3.org/2000/svg" x1 = "14" y1 = "1" x2 = "14" y2 = "4" /> < title >
        { title } < / title > < / svg >
    }
}
