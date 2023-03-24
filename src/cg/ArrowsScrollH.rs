#[cfg(feature = "CgArrowsScrollH")]
use leptos::*;
#[cfg(feature = "CgArrowsScrollH")]
///This icon requires the feature `CgArrowsScrollH` to be enabled.
#[component]
pub fn ArrowsScrollH(
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
        "M15.3052 12L18.1299 9.17526L16.7157 7.76105L13.891 10.5858L13.8873 10.5821L12.4731 11.9963L12.4768 12L12.4731 12.0037L13.8873 13.4179L13.891 13.4142L16.7157 16.239L18.1299 14.8248L15.3052 12Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10.1091 10.5858L10.1128 10.5821L11.527 11.9963L11.5233 12L11.527 12.0037L10.1128 13.4179L10.1091 13.4142L7.28433 16.239L5.87012 14.8248L8.69487 12L5.87012 9.17526L7.28433 7.76105L10.1091 10.5858Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
