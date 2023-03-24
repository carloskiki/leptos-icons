#[cfg(feature = "CgFormatBold")]
use leptos::*;
#[cfg(feature = "CgFormatBold")]
///This icon requires the feature `CgFormatBold` to be enabled.
#[component]
pub fn FormatBold(
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
        "M12.946 10.7833L11.2754 12.209L13.4476 12.533C14.8919 12.7485 16 13.9957 16 15.5C16 13.8431 14.6569 12.5 13 12.5H9H8V11.5H9H11C12.6283 11.5 13.9536 10.2028 13.9988 8.58539C13.9743 9.4647 13.5724 10.2488 12.946 10.7833Z"
        stroke = "currentColor" stroke - width = "2" /> < title > { title } < / title > <
        / svg >
    }
}
