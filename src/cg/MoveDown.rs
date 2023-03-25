#[cfg(feature = "CgMoveDown")]
use leptos::*;
#[cfg(feature = "CgMoveDown")]
///This icon requires the feature `CgMoveDown` to be enabled.
#[component]
pub fn MoveDown(
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
        "M7 5H9V13H7V5Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 5H17V13H15V5Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M11.0001 5H13.0001V14.9999H16.0355L12.0356 19.071L8.03564 14.9999H11.0001V5Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
