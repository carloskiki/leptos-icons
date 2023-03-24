#[cfg(feature = "CgSwapVertical")]
use leptos::*;
#[cfg(feature = "CgSwapVertical")]
///This icon requires the feature `CgSwapVertical` to be enabled.
#[component]
pub fn SwapVertical(
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
        "M12 16H13.5L13.5 10H15.5L15.5 16H17L14.5 19L12 16Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 8H9.5L9.5 14H11.5L11.5 8H13L10.5 5L8 8Z" fill = "currentColor" /> < title > {
        title } < / title > < / svg >
    }
}
