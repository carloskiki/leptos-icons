#[cfg(feature = "CgArrowsExpandDownRight")]
use leptos::*;
#[cfg(feature = "CgArrowsExpandDownRight")]
///This icon requires the feature `CgArrowsExpandDownRight` to be enabled.
#[component]
pub fn ArrowsExpandDownRight(
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
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M11 5C11 3.89543 10.1046 3 9 3H5C3.89543 3 3 3.89543 3 5V9C3 10.1046 3.89543 11 5 11H9C10.1046 11 11 10.1046 11 9V5ZM9 5H5V9H9V5Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M19 13H21V21H13V19H17.5858L12.2218 13.6361C11.8313 13.2456 11.8313 12.6124 12.2218 12.2219C12.6124 11.8314 13.2455 11.8314 13.6361 12.2219L19 17.5858V13Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
