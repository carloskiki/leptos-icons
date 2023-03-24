#[cfg(feature = "BiSolidLemon")]
use leptos::*;
#[cfg(feature = "BiSolidLemon")]
///This icon requires the feature `BiSolidLemon` to be enabled.
#[component]
pub fn Lemon(
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
        "M21.45 8.74A2.23 2.23 0 0 1 21.64 7a3.51 3.51 0 0 0 .24-2.47 3.55 3.55 0 0 0-2.45-2.45 3.51 3.51 0 0 0-2.43.28 2.23 2.23 0 0 1-1.7.19 10.07 10.07 0 0 0-6.53 0 9.87 9.87 0 0 0-6.23 6.18 10.07 10.07 0 0 0 0 6.53A2.23 2.23 0 0 1 2.36 17a3.51 3.51 0 0 0-.24 2.47 3.55 3.55 0 0 0 2.45 2.45A3.51 3.51 0 0 0 7 21.64a2.23 2.23 0 0 1 1.7-.19A9.83 9.83 0 0 0 12 22a10.33 10.33 0 0 0 3.27-.54 9.87 9.87 0 0 0 6.19-6.19 10.07 10.07 0 0 0-.01-6.53zM12 7a5 5 0 0 0-5 5H5a7 7 0 0 1 7-7z"
        /> < title > { title } < / title > < / svg >
    }
}
