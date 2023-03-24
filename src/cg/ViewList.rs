#[cfg(feature = "CgViewList")]
use leptos::*;
#[cfg(feature = "CgViewList")]
///This icon requires the feature `CgViewList` to be enabled.
#[component]
pub fn ViewList(
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
        "M5 5C3.34315 5 2 6.34315 2 8V16C2 17.6569 3.34315 19 5 19H19C20.6569 19 22 17.6569 22 16V8C22 6.34315 20.6569 5 19 5H5ZM7 7H5C4.44772 7 4 7.44772 4 8V9H7V7ZM9 7V9H20V8C20 7.44771 19.5523 7 19 7H9ZM7 11H4V13H7V11ZM9 13V11H20V13H9ZM7 15H4V16C4 16.5523 4.44772 17 5 17H7V15ZM9 17V15H20V16C20 16.5523 19.5523 17 19 17H9Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
