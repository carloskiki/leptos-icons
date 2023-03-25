#[cfg(feature = "CgArrowsExpandDownLeft")]
use leptos::*;
#[cfg(feature = "CgArrowsExpandDownLeft")]
///This icon requires the feature `CgArrowsExpandDownLeft` to be enabled.
#[component]
pub fn ArrowsExpandDownLeft(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M13 5C13 3.89543 13.8954 3 15 3H19C20.1046 3 21 3.89543 21 5V9C21 10.1046 20.1046 11 19 11H15C13.8954 11 13 10.1046 13 9V5ZM15 5H19V9H15V5Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5 13H3V21H11V19H6.41424L11.7782 13.6361C12.1687 13.2456 12.1687 12.6124 11.7782 12.2219C11.3876 11.8314 10.7545 11.8314 10.3639 12.2219L5 17.5858V13Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
