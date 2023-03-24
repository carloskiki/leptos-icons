#[cfg(feature = "SiDash")]
use leptos::*;
#[cfg(feature = "SiDash")]
///This icon requires the feature `SiDash` to be enabled.
#[component]
pub fn Dash(
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
        "M3.21 9.967C.922 9.967.595 11.457.38 12.36.093 13.538 0 14.02 0 14.02h8.947c2.29 0 2.617-1.492 2.832-2.394.285-1.178.379-1.66.379-1.66zM15.72 2.26H6.982L6.26 6.307l7.884.01c3.885 0 5.03 1.41 4.997 3.748-.019 1.196-.537 3.225-.762 3.884-.598 1.753-1.827 3.749-6.435 3.744l-7.666-.004-.725 4.052h8.718c3.075 0 4.38-.36 5.767-.995 3.071-1.426 4.9-4.455 5.633-8.41C24.76 6.448 23.403 2.26 15.72 2.26z"
        /> < title > { title } < / title > < / svg >
    }
}
