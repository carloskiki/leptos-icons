#[cfg(feature = "IoLogoFirebase")]
use leptos::*;
#[cfg(feature = "IoLogoFirebase")]
///This icon requires the feature `IoLogoFirebase` to be enabled.
#[component]
pub fn LogoFirebase(
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
        stroke_witdh = "0" style = style id = "icons" viewBox = "0 0 512 512" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" id
        = "icon" d =
        "M93.19,329.38,140.64,25.31c1.64-10.37,15.55-12.82,20.46-3.55l51,95.45ZM432,400,385.26,123.21a11,11,0,0,0-18.54-6L80,400l159.36,91.91a33.18,33.18,0,0,0,31.91,0ZM302.36,158.93,265.82,89.39a10.86,10.86,0,0,0-19.36,0L85.83,375.74Z"
        /> < title > { title } < / title > < / svg >
    }
}
