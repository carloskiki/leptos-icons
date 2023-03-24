#[cfg(feature = "SiCachet")]
use leptos::*;
#[cfg(feature = "SiCachet")]
///This icon requires the feature `SiCachet` to be enabled.
#[component]
pub fn Cachet(
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
        "M11.746.254C5.265.254 0 5.519 0 12c0 6.481 5.265 11.746 11.746 11.746 6.482 0 11.746-5.265 11.746-11.746 0-1.44-.26-2.82-.734-4.097l-.264-.709-1.118 1.118.1.288c.373 1.064.575 2.207.575 3.4a10.297 10.297 0 01-10.305 10.305A10.297 10.297 0 011.441 12 10.297 10.297 0 0111.746 1.695c1.817 0 3.52.47 5.002 1.293l.32.178 1.054-1.053-.553-.316A11.699 11.699 0 0011.746.254zM22.97.841l-13.92 13.92-3.722-3.721-1.031 1.03 4.752 4.753L24 1.872z"
        /> < title > { title } < / title > < / svg >
    }
}
