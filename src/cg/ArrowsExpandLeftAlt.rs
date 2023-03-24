#[cfg(feature = "CgArrowsExpandLeftAlt")]
use leptos::*;
#[cfg(feature = "CgArrowsExpandLeftAlt")]
///This icon requires the feature `CgArrowsExpandLeftAlt` to be enabled.
#[component]
pub fn ArrowsExpandLeftAlt(
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
        "http://www.w3.org/2000/svg" d =
        "M10.1005 2.10052V4.10052H5.51471L11.293 9.87878L9.87875 11.293L4.10046 5.51471L4.10046 10.1005H2.10046L2.10046 2.10052H10.1005Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M21.8995 13.8995H19.8995V18.4853L14.1212 12.707L12.707 14.1213L18.4853 19.8995H13.8995V21.8995H21.8995V13.8995Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M16.2426 9.1716L14.8284 7.75739L7.7573 14.8285L9.17151 16.2427L16.2426 9.1716Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
