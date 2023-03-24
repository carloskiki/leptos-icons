#[cfg(feature = "CgBorderLeft")]
use leptos::*;
#[cfg(feature = "CgBorderLeft")]
///This icon requires the feature `CgBorderLeft` to be enabled.
#[component]
pub fn BorderLeft(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 8V16H9L9 19H19L19 5L9 5V8H16Z" fill =
        "currentColor" fill - opacity = "0.3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7 5L7 19H4L4 5L7 5Z" fill = "currentColor" />
        < title > { title } < / title > < / svg >
    }
}
