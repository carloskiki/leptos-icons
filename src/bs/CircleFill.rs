#[cfg(feature = "BsCircleFill")]
use leptos::*;
#[cfg(feature = "BsCircleFill")]
///This icon requires the feature `BsCircleFill` to be enabled.
#[component]
pub fn CircleFill(
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
        class = "bi bi-circle-fill" viewBox = "0 0 16 16" width = { size.clone() } height
        = { size } > < circle xmlns = "http://www.w3.org/2000/svg" cx = "8" cy = "8" r =
        "8" /> < title > { title } < / title > < / svg >
    }
}
