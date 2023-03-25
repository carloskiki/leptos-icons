#[cfg(feature = "CgMinimize")]
use leptos::*;
#[cfg(feature = "CgMinimize")]
///This icon requires the feature `CgMinimize` to be enabled.
#[component]
pub fn Minimize(
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
        "M9 9H3V7H7V3H9V9Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 15H3V17H7V21H9V15Z" fill = "currentColor"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M21 15H15V21H17V17H21V15Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15 9.00012H21V7.00012H17V3.00012H15V9.00012Z" fill = "currentColor" /> < title
        > { title } < / title > < / svg >
    }
}
