#[cfg(feature = "CgCodeClimate")]
use leptos::*;
#[cfg(feature = "CgCodeClimate")]
///This icon requires the feature `CgCodeClimate` to be enabled.
#[component]
pub fn CodeClimate(
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
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M9.49506 8.1109L3.1311 14.4749L4.54532 15.8891L9.49506 10.9393L14.4448 15.8891L15.859 14.4749L9.49506 8.1109Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M14.5049 8.11092L12.5317 10.0842L13.9503 11.494L14.5049 10.9393L19.4547 15.8891L20.8689 14.4749L14.5049 8.11092Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
