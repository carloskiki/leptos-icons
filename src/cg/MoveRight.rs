#[cfg(feature = "CgMoveRight")]
use leptos::*;
#[cfg(feature = "CgMoveRight")]
///This icon requires the feature `CgMoveRight` to be enabled.
#[component]
pub fn MoveRight(
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
        "http://www.w3.org/2000/svg" d = "M5 17V15H13V17H5Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M5 9V7H13V9H5Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5 12.9999V10.9999H14.9999V7.96454L19.071 11.9644L14.9999 15.9644V12.9999H5Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
