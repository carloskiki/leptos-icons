#[cfg(feature = "CgArrowsShrinkV")]
use leptos::*;
#[cfg(feature = "CgArrowsShrinkV")]
///This icon requires the feature `CgArrowsShrinkV` to be enabled.
#[component]
pub fn ArrowsShrinkV(
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
        "M17 1V3L7 3V1L17 1Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M16.2427 8.44772L14.8285 9.86194L13 8.03347L13 15.9665L14.8285 14.138L16.2427 15.5522L12 19.7949L7.75742 15.5522L9.17163 14.138L11 15.9664L11 8.03357L9.17163 9.86194L7.75742 8.44772L12 4.20508L16.2427 8.44772Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M17 23V21H7V23H17Z" fill = "currentColor" /> < title > { title } < / title > < /
        svg >
    }
}
