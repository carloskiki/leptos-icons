#[cfg(feature = "BsSignDeadEndFill")]
use leptos::*;
#[cfg(feature = "BsSignDeadEndFill")]
///This icon requires the feature `BsSignDeadEndFill` to be enabled.
#[component]
pub fn SignDeadEndFill(
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
        stroke_witdh = "0" style = style width = "16" height = "16" fill = "currentColor"
        class = "bi bi-sign-dead-end-fill" viewBox = "0 0 16 16" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M5.116 6.28h.32c.395 0 .582.24.582.722 0 .48-.186.718-.581.718h-.321V6.28Zm3.636.066.268.845h-.552l.27-.845h.014Zm1.327-.066h.32c.394 0 .582.24.582.722 0 .48-.186.718-.582.718h-.32V6.28Zm-.792 3h.32c.395 0 .582.24.582.722 0 .48-.186.718-.581.718h-.32V9.28Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9.05.435c-.58-.58-1.52-.58-2.1 0L.436 6.95c-.58.58-.58 1.519 0 2.098l6.516 6.516c.58.58 1.519.58 2.098 0l6.516-6.516c.58-.58.58-1.519 0-2.098L9.05.435ZM4.782 6h.69c.596 0 .886.355.886.998S6.066 8 5.473 8h-.69V6ZM7.82 7.72V8H6.571V6H7.82v.28h-.917v.57h.863v.268h-.863v.602h.917Zm.397.28h-.34l.688-2h.371l.689 2h-.352l-.177-.554h-.702L8.216 8Zm1.53-2h.69c.596 0 .886.355.886.998S11.03 8 10.436 8h-.69V6Zm-2.923 4.72V11H5.575V9h1.248v.28h-.917v.57h.863v.268h-.863v.602h.917Zm.572.28h-.32V9h.294l.933 1.436h.014V9h.32v2h-.292l-.936-1.44h-.013V11Zm1.56-2h.69c.596 0 .886.355.886.998S10.238 11 9.645 11h-.69V9Z"
        /> < title > { title } < / title > < / svg >
    }
}
