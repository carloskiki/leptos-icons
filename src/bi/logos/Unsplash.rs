#[cfg(feature = "BiLogosUnsplash")]
use leptos::*;
#[cfg(feature = "BiLogosUnsplash")]
///This icon requires the feature `BiLogosUnsplash` to be enabled.
#[component]
pub fn Unsplash(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M8.625 8.063V3h6.75v5.063h-6.75zm6.75 2.812H21V21H3V10.875h5.625v5.063h6.75v-5.063z"
        /> < title > { title } < / title > < / svg >
    }
}
