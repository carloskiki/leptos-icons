#[cfg(feature = "CgMathPercent")]
use leptos::*;
#[cfg(feature = "CgMathPercent")]
///This icon requires the feature `CgMathPercent` to be enabled.
#[component]
pub fn MathPercent(
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
        "M16.2426 6.34319C16.6331 5.95266 17.2663 5.95266 17.6568 6.34319C18.0474 6.73371 18.0474 7.36688 17.6568 7.7574L7.75734 17.6569C7.36681 18.0474 6.73365 18.0474 6.34313 17.6569C5.9526 17.2664 5.9526 16.6332 6.34313 16.2427L16.2426 6.34319Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9.87866 9.87872C9.09761 10.6598 7.83128 10.6598 7.05023 9.87872C6.26918 9.09767 6.26918 7.83134 7.05023 7.05029C7.83128 6.26924 9.09761 6.26924 9.87866 7.05029C10.6597 7.83134 10.6597 9.09767 9.87866 9.87872Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M14.1213 16.9498C14.9023 17.7308 16.1687 17.7308 16.9497 16.9498C17.7308 16.1687 17.7308 14.9024 16.9497 14.1214C16.1687 13.3403 14.9023 13.3403 14.1213 14.1214C13.3403 14.9024 13.3403 16.1687 14.1213 16.9498Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
