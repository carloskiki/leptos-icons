#[cfg(feature = "BiSolidLeftDownArrowCircle")]
use leptos::*;
#[cfg(feature = "BiSolidLeftDownArrowCircle")]
///This icon requires the feature `BiSolidLeftDownArrowCircle` to be enabled.
#[component]
pub fn LeftDownArrowCircle(
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
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M19.071 19.071c3.898-3.899 3.898-10.244 0-14.143-3.899-3.899-10.244-3.898-14.143 0-3.898 3.899-3.899 10.243 0 14.143 3.9 3.899 10.244 3.899 14.143 0zM8.464 8.464l2.829 2.829 3.535-3.536 1.414 1.414-3.535 3.536 2.828 2.829H8.464V8.464z"
        /> < title > { title } < / title > < / svg >
    }
}
