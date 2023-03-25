#[cfg(feature = "HiMdSolidUser")]
use leptos::*;
#[cfg(feature = "HiMdSolidUser")]
///This icon requires the feature `HiMdSolidUser` to be enabled.
#[component]
pub fn User(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M10 8C11.6569 8 13 6.65685 13 5C13 3.34315 11.6569 2 10 2C8.34315 2 7 3.34315 7 5C7 6.65685 8.34315 8 10 8Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.46517 14.4935C3.27029 15.0016 3.44435 15.571 3.8742 15.9046C5.56656 17.218 7.69202 18 10.0001 18C12.3106 18 14.438 17.2164 16.1312 15.9006C16.5608 15.5667 16.7345 14.9971 16.5393 14.4892C15.5301 11.8635 12.9842 10 10.0031 10C7.02032 10 4.47329 11.8656 3.46517 14.4935Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
