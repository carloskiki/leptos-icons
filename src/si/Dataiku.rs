#[cfg(feature = "SiDataiku")]
use leptos::*;
#[cfg(feature = "SiDataiku")]
///This icon requires the feature `SiDataiku` to be enabled.
#[component]
pub fn Dataiku(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M12 0a12 12 0 1 0 12 12A12 12 0 0 0 12 0zm6.527 15.34H12.5v-.934h6.026zm-.739-8.73s-.412.543-.193 1.995c.41 2.724-1.02 5.15-3.56 5.15h-1.87s-1.835-.092-2.933 1.01c-3.263 3.269-4.04 4.116-4.274 4.233-.15.08-.188-.093-.188-.093l9.644-11.891c-.203-2.145 2.34-2.715 3.278-1.13l.884-.248zm-1.599-.614a.476.476 0 1 0 .47.474.476.476 0 0 0-.47-.474z"
        /> < title > { title } < / title > < / svg >
    }
}
