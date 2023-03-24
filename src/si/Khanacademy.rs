#[cfg(feature = "SiKhanacademy")]
use leptos::*;
#[cfg(feature = "SiKhanacademy")]
///This icon requires the feature `SiKhanacademy` to be enabled.
#[component]
pub fn Khanacademy(
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
        "M21.724 4.973L13.418.328a3.214 3.214 0 0 0-2.828 0L2.276 4.973A3.05 3.05 0 0 0 .862 7.371v9.256a3.05 3.05 0 0 0 1.414 2.4l8.306 4.645a3.214 3.214 0 0 0 2.828 0l8.314-4.645a3.05 3.05 0 0 0 1.414-2.4V7.373a3.05 3.05 0 0 0-1.414-2.4zM12 4.921a2.571 2.571 0 1 1 .001 5.143A2.571 2.571 0 0 1 12 4.92zm3.094 13.627a9.119 9.119 0 0 1-3.103.549 8.972 8.972 0 0 1-3.076-.55 8.493 8.493 0 0 1-5.486-7.987v-.857c4.646.017 8.074 3.823 8.074 8.51v.198h.926v-.197c0-4.688 3.445-8.51 8.056-8.51.026.29.043.582.086.856a8.502 8.502 0 0 1-5.477 7.988z"
        /> < title > { title } < / title > < / svg >
    }
}
