#[cfg(feature = "BiSolidHomeHeart")]
use leptos::*;
#[cfg(feature = "BiSolidHomeHeart")]
///This icon requires the feature `BiSolidHomeHeart` to be enabled.
#[component]
pub fn HomeHeart(
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
        "M3 14h2v7a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1v-7h2a.998.998 0 0 0 .913-.593.998.998 0 0 0-.17-1.076l-9-10c-.379-.422-1.107-.422-1.486 0l-9 10A1 1 0 0 0 3 14zm5.653-2.359a2.224 2.224 0 0 1 3.125 0l.224.22.223-.22a2.225 2.225 0 0 1 3.126 0 2.13 2.13 0 0 1 0 3.07L12.002 18l-3.349-3.289a2.13 2.13 0 0 1 0-3.07z"
        /> < title > { title } < / title > < / svg >
    }
}
