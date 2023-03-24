#[cfg(feature = "IoCafeSharp")]
use leptos::*;
#[cfg(feature = "IoCafeSharp")]
///This icon requires the feature `IoCafeSharp` to be enabled.
#[component]
pub fn CafeSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M448,64H80V318.34a19.83,19.83,0,0,0,5.86,14.14l29.65,29.66A19.87,19.87,0,0,0,129.66,368H334.34a19.87,19.87,0,0,0,14.15-5.86l29.65-29.66A19.83,19.83,0,0,0,384,318.34V192h32a16,16,0,0,0,11.31-4.69l32-32A16,16,0,0,0,464,144V80A16,16,0,0,0,448,64Zm-16,73.37L409.37,160H384V96h48Z"
        />< rect xmlns = "http://www.w3.org/2000/svg" x = "48" y = "400" width = "368"
        height = "32" /> < title > { title } < / title > < / svg >
    }
}
