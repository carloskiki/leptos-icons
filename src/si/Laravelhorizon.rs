#[cfg(feature = "SiLaravelhorizon")]
use leptos::*;
#[cfg(feature = "SiLaravelhorizon")]
///This icon requires the feature `SiLaravelhorizon` to be enabled.
#[component]
pub fn Laravelhorizon(
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
        "M20.486 3.516C15.8-1.171 8.202-1.172 3.516 3.513A11.963 11.963 0 0 0 0 11.998a11.975 11.975 0 0 0 4.2 9.13h.01a12 12 0 0 0 16.274-.642c4.687-4.685 4.688-12.283.002-16.97zM16 13.998c-4 0-4-4-8-4-2.5 0-3.44 1.565-4.765 2.74H3.23a8.801 8.801 0 0 1 17.54-1.48c-1.33 1.175-2.27 2.74-4.77 2.74z"
        /> < title > { title } < / title > < / svg >
    }
}
