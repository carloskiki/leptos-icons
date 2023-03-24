#[cfg(feature = "OcSmAccessibilityInset")]
use leptos::*;
#[cfg(feature = "OcSmAccessibilityInset")]
///This icon requires the feature `OcSmAccessibilityInset` to be enabled.
#[component]
pub fn AccessibilityInset(
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
        "M8 0a8 8 0 1 1 0 16A8 8 0 0 1 8 0Zm2 4a2 2 0 1 0-2.95 1.76 1.87 1.87 0 0 0-.32.24H3.75a.75.75 0 0 0 0 1.5h2.363l-.607 5.67a.75.75 0 1 0 1.49.16l.25-2.33h1.508l.25 2.33a.75.75 0 0 0 1.492-.16L9.888 7.5h2.362a.75.75 0 0 0 0-1.5H9.27a1.98 1.98 0 0 0-.32-.24A2 2 0 0 0 10 4Z"
        /> < title > { title } < / title > < / svg >
    }
}
