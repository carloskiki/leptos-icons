#[cfg(feature = "FaSolidHandPointDown")]
use leptos::*;
#[cfg(feature = "FaSolidHandPointDown")]
///This icon requires the feature `FaSolidHandPointDown` to be enabled.
#[component]
pub fn HandPointDown(
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
        stroke_witdh = "0" style = style viewBox = "0 0 384 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M32 480c0 17.7 14.3 32 32 32s32-14.3 32-32V272H32V480zM224 320c0 17.7 14.3 32 32 32s32-14.3 32-32V256c0-17.7-14.3-32-32-32s-32 14.3-32 32v64zm-64 64c17.7 0 32-14.3 32-32V304c0-17.7-14.3-32-32-32s-32 14.3-32 32v48c0 17.7 14.3 32 32 32zm160-96c0 17.7 14.3 32 32 32s32-14.3 32-32V224c0-17.7-14.3-32-32-32s-32 14.3-32 32v64zm-96-88l0 .6c9.4-5.4 20.3-8.6 32-8.6c13.2 0 25.4 4 35.6 10.8c8.7-24.9 32.5-42.8 60.4-42.8c11.7 0 22.6 3.1 32 8.6V160C384 71.6 312.4 0 224 0H162.3C119.8 0 79.1 16.9 49.1 46.9L37.5 58.5C13.5 82.5 0 115.1 0 149v27c0 35.3 28.7 64 64 64h88c22.1 0 40-17.9 40-40s-17.9-40-40-40H96c-8.8 0-16-7.2-16-16s7.2-16 16-16h56c39.8 0 72 32.2 72 72z"
        /> < title > { title } < / title > < / svg >
    }
}
