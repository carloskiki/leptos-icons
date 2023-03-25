#[cfg(feature = "CgArrowsExpandUpLeft")]
use leptos::*;
#[cfg(feature = "CgArrowsExpandUpLeft")]
///This icon requires the feature `CgArrowsExpandUpLeft` to be enabled.
#[component]
pub fn ArrowsExpandUpLeft(
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
        "M5 11H3L3 3L11 3V5L6.41419 5L11.7781 10.3639C12.1686 10.7545 12.1686 11.3876 11.7781 11.7782C11.3876 12.1687 10.7544 12.1687 10.3639 11.7782L5 6.41424L5 11Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M19 13C20.1046 13 21 13.8954 21 15V19C21 20.1046 20.1046 21 19 21H15C13.8954 21 13 20.1046 13 19V15C13 13.8954 13.8954 13 15 13H19ZM19 15V19H15V15H19Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
