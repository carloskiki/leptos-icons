#[cfg(feature = "VsArrowCircleLeft")]
use leptos::*;
#[cfg(feature = "VsArrowCircleLeft")]
///This icon requires the feature `VsArrowCircleLeft` to be enabled.
#[component]
pub fn ArrowCircleLeft(
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
        fill = "currentColor" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M7.91926 10.6311L5.77984 8.49167L11.532 8.49167L11.532 7.49167L5.85271 7.49167L7.91926 5.42511L7.21216 4.718L4.25561 7.67455L4.25561 8.38165L7.21216 11.3382L7.91926 10.6311Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 2C11.3137 2 14 4.68629 14 8C14 11.3137 11.3137 14 8 14C4.68629 14 2 11.3137 2 8C2 4.68629 4.68629 2 8 2ZM8 3C5.23858 3 3 5.23858 3 8C3 10.7614 5.23858 13 8 13C10.7614 13 13 10.7614 13 8C13 5.23858 10.7614 3 8 3Z"
        /> < title > { title } < / title > < / svg >
    }
}
