#[cfg(feature = "BiRegularMath")]
use leptos::*;
#[cfg(feature = "BiRegularMath")]
///This icon requires the feature `BiRegularMath` to be enabled.
#[component]
pub fn Math(
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
        "M7 2H5v3H2v2h3v3h2V7h3V5H7V2zm7 3h8v2h-8zm0 10h8v2h-8zm0 4h8v2h-8zm-5.71-4.71L6 16.59l-2.29-2.3-1.42 1.42L4.59 18l-2.3 2.29 1.42 1.42L6 19.41l2.29 2.3 1.42-1.42L7.41 18l2.3-2.29-1.42-1.42z"
        /> < title > { title } < / title > < / svg >
    }
}
