#[cfg(feature = "BiLogosDeezer")]
use leptos::*;
#[cfg(feature = "BiLogosDeezer")]
///This icon requires the feature `BiLogosDeezer` to be enabled.
#[component]
pub fn Deezer(
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
        "M17.68 5.47H22V8h-4.32zm0 3.51H22v2.53h-4.32zm0 3.51H22v2.53h-4.32zM2 16h4.32v2.53H2zm5.22 0h4.32v2.53H7.22zm5.23 0h4.32v2.53h-4.32zm5.23 0H22v2.53h-4.32zm-5.23-3.51h4.32v2.53h-4.32zm-5.23 0h4.32v2.53H7.22zm0-3.51h4.32v2.53H7.22z"
        /> < title > { title } < / title > < / svg >
    }
}
