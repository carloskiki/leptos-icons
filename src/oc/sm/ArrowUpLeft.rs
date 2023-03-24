#[cfg(feature = "OcSmArrowUpLeft")]
use leptos::*;
#[cfg(feature = "OcSmArrowUpLeft")]
///This icon requires the feature `OcSmArrowUpLeft` to be enabled.
#[component]
pub fn ArrowUpLeft(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M3.96 4.75A.75.75 0 0 1 4.71 4h6.01a.75.75 0 0 1 0 1.5h-4.2l5.26 5.26a.75.75 0 0 1-1.06 1.061l-5.26-5.26v4.2a.75.75 0 0 1-1.5 0Z"
        /> < title > { title } < / title > < / svg >
    }
}
