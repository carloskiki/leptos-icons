#[cfg(feature = "BsSignMergeLeftFill")]
use leptos::*;
#[cfg(feature = "BsSignMergeLeftFill")]
///This icon requires the feature `BsSignMergeLeftFill` to be enabled.
#[component]
pub fn SignMergeLeftFill(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    let style = format!("{} color: {};", style, color);
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "16" height = "16" fill = "currentColor"
        class = "bi bi-sign-merge-left-fill" viewBox = "0 0 16 16" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M9.05.435c-.58-.58-1.52-.58-2.1 0L.436 6.95c-.58.58-.58 1.519 0 2.098l6.516 6.516c.58.58 1.519.58 2.098 0l6.516-6.516c.58-.58.58-1.519 0-2.098L9.05.435ZM7.25 6H6.034a.25.25 0 0 1-.192-.41l1.966-2.36a.25.25 0 0 1 .384 0l1.966 2.36a.25.25 0 0 1-.192.41H8.75v6h-1.5V8.823c-.551.686-1.229 1.363-1.88 2.015l-.016.016-.708-.708c.757-.756 1.48-1.48 2.016-2.196.25-.333.448-.649.588-.95V6Z"
        /> < title > { title } < / title > < / svg >
    }
}
