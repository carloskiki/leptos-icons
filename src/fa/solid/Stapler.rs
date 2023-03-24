#[cfg(feature = "FaSolidStapler")]
use leptos::*;
#[cfg(feature = "FaSolidStapler")]
///This icon requires the feature `FaSolidStapler` to be enabled.
#[component]
pub fn Stapler(
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
        stroke_witdh = "0" style = style viewBox = "0 0 640 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M640 299.3V304 432c0 26.5-21.5 48-48 48H512 448 64c-17.7 0-32-14.3-32-32s14.3-32 32-32H448V368H96c-17.7 0-32-14.3-32-32V219.4L33.8 214C14.2 210.5 0 193.5 0 173.7c0-8.9 2.9-17.5 8.2-24.6l35.6-47.5C76.7 57.8 128.2 32 182.9 32c27 0 53.6 6.3 77.8 18.4L586.9 213.5C619.5 229.7 640 263 640 299.3zM448 304V288L128 230.9V304H448z"
        /> < title > { title } < / title > < / svg >
    }
}
