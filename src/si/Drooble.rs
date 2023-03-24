#[cfg(feature = "SiDrooble")]
use leptos::*;
#[cfg(feature = "SiDrooble")]
///This icon requires the feature `SiDrooble` to be enabled.
#[component]
pub fn Drooble(
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
        "M24 11.986a7.599 7.599 0 0 0-7-7.559v7.574a5 5 0 0 1-10 0c0-3.604 3.707-6.022 7-4.583V.17C6.615-1.069 0 4.63 0 12c0 6.628 5.373 12 12 12 6.628 0 12-5.372 12-12v-.014m-14 .015a2 2 0 1 0 4 0 2 2 0 0 0-4 0m14-.015a7.599 7.599 0 0 0-7-7.559v7.574a5 5 0 0 1-10 0c0-3.604 3.707-6.022 7-4.583V.17C6.615-1.069 0 4.63 0 12c0 6.628 5.373 12 12 12 6.628 0 12-5.372 12-12v-.014m-14 .015a2 2 0 1 0 4 0 2 2 0 0 0-4 0Z"
        /> < title > { title } < / title > < / svg >
    }
}
