#[cfg(feature = "FaBrandsJira")]
use leptos::*;
#[cfg(feature = "FaBrandsJira")]
///This icon requires the feature `FaBrandsJira` to be enabled.
#[component]
pub fn Jira(
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
        stroke_witdh = "0" style = style viewBox = "0 0 496 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M490 241.7C417.1 169 320.6 71.8 248.5 0 83 164.9 6 241.7 6 241.7c-7.9 7.9-7.9 20.7 0 28.7C138.8 402.7 67.8 331.9 248.5 512c379.4-378 15.7-16.7 241.5-241.7 8-7.9 8-20.7 0-28.6zm-241.5 90l-76-75.7 76-75.7 76 75.7-76 75.7z"
        /> < title > { title } < / title > < / svg >
    }
}
