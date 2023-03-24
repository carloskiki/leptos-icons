#[cfg(feature = "FaSolidTentArrowsDown")]
use leptos::*;
#[cfg(feature = "FaSolidTentArrowsDown")]
///This icon requires the feature `FaSolidTentArrowsDown` to be enabled.
#[component]
pub fn TentArrowsDown(
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
        stroke_witdh = "0" style = style viewBox = "0 0 576 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M201.8 111.9c-8.9-9.9-24-10.7-33.9-1.8l-39.9 36L128 24c0-13.3-10.7-24-24-24S80 10.7 80 24l0 122.1-39.9-36c-9.9-8.9-25-8.1-33.9 1.8s-8.1 25 1.8 33.9l80 72c9.1 8.2 23 8.2 32.1 0l80-72c9.9-8.9 10.7-24 1.8-33.9zm352 0c-8.9-9.9-24-10.7-33.9-1.8l-39.9 36V24c0-13.3-10.7-24-24-24s-24 10.7-24 24V146.1l-39.9-36c-9.9-8.9-25-8.1-33.9 1.8s-8.1 25 1.8 33.9l80 72c9.1 8.2 23 8.2 32.1 0l80-72c9.9-8.9 10.7-24 1.8-33.9zM299.4 166.5c-11.5-8.7-27.3-8.7-38.8 0l-168 128c-6.6 5-11 12.5-12.3 20.7l-24 160c-1.4 9.2 1.3 18.6 7.4 25.6S78.7 512 88 512H280V352l96 160h96c9.3 0 18.2-4.1 24.2-11.1s8.8-16.4 7.4-25.6l-24-160c-1.2-8.2-5.6-15.7-12.3-20.7l-168-128z"
        /> < title > { title } < / title > < / svg >
    }
}
