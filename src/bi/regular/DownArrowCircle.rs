#[cfg(feature = "BiRegularDownArrowCircle")]
use leptos::*;
#[cfg(feature = "BiRegularDownArrowCircle")]
///This icon requires the feature `BiRegularDownArrowCircle` to be enabled.
#[component]
pub fn DownArrowCircle(
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
        "M12 1.993C6.486 1.994 2 6.48 2 11.994c0 5.513 4.486 9.999 10 10 5.514 0 10-4.486 10-10s-4.485-10-10-10.001zm0 18.001c-4.411-.001-8-3.59-8-8 0-4.411 3.589-8 8-8.001 4.411.001 8 3.59 8 8.001s-3.589 8-8 8z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M13 8h-2v4H7.991l4.005 4.005L16 12h-3z" /> < title > { title } < / title > < /
        svg >
    }
}
