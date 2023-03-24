#[cfg(feature = "SiD3dotjs")]
use leptos::*;
#[cfg(feature = "SiD3dotjs")]
///This icon requires the feature `SiD3dotjs` to be enabled.
#[component]
pub fn D3dotjs(
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
        "M13.312 12C13.312 5.718 8.22.625 1.937.625H0v5h1.938c3.521 0 6.375 2.854 6.375 6.375s-2.854 6.375-6.375 6.375H0v5h1.938c6.281 0 11.374-5.093 11.374-11.375zM24 7.563C24 3.731 20.893.625 17.062.625h-8a13.4154 13.4154 0 0 1 4.686 5h3.314c1.069 0 1.938.868 1.938 1.938 0 1.07-.869 1.938-1.938 1.938h-1.938c.313 1.652.313 3.348 0 5h1.938c1.068 0 1.938.867 1.938 1.938s-.869 1.938-1.938 1.938h-3.314a13.4154 13.4154 0 0 1-4.686 5h8c1.621 0 3.191-.568 4.438-1.605 2.943-2.45 3.346-6.824.895-9.77A6.9459 6.9459 0 0 0 24 7.563z"
        /> < title > { title } < / title > < / svg >
    }
}
