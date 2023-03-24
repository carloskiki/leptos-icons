#[cfg(feature = "CgFigma")]
use leptos::*;
#[cfg(feature = "CgFigma")]
///This icon requires the feature `CgFigma` to be enabled.
#[component]
pub fn Figma(
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
        "M8.5 2C6.84315 2 5.5 3.34315 5.5 5C5.5 6.65685 6.84315 8 8.5 8H15.5C17.1569 8 18.5 6.65685 18.5 5C18.5 3.34315 17.1569 2 15.5 2H8.5Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15.5 9C13.8431 9 12.5 10.3431 12.5 12C12.5 13.6569 13.8431 15 15.5 15C17.1569 15 18.5 13.6569 18.5 12C18.5 10.3431 17.1569 9 15.5 9Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5.5 12C5.5 10.3431 6.84315 9 8.5 9H11.5V15H8.5C6.84315 15 5.5 13.6569 5.5 12Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8.5 16C6.84315 16 5.5 17.3431 5.5 19C5.5 20.6569 6.84315 22 8.5 22C10.1569 22 11.5 20.6569 11.5 19V16H8.5Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
