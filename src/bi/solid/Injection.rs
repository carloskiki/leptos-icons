#[cfg(feature = "BiSolidInjection")]
use leptos::*;
#[cfg(feature = "BiSolidInjection")]
///This icon requires the feature `BiSolidInjection` to be enabled.
#[component]
pub fn Injection(
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
        "M12 5.61 9.24 8.35l3.31 3.31-1.06 1.06-3.31-3.31-1.77 1.77 3.31 3.31-1.06 1.06-3.31-3.31-2 2A2 2 0 0 0 3 16.66l1 1.89-2.25 2.29 1.41 1.41L5.45 20l1.89 1a2 2 0 0 0 1 .26 2 2 0 0 0 1.42-.59L18.39 12zm7.8 3.59-1.79-1.8 1.42-1.41 1.41 1.41 1.41-1.41-4.24-4.24-1.41 1.41 1.41 1.42-1.41 1.41-1.8-1.79-1.74-1.75-1.41 1.42 1.03 1.03v.01l6.41 6.41h.01l1.03 1.03 1.42-1.41-1.74-1.74h-.01z"
        /> < title > { title } < / title > < / svg >
    }
}
