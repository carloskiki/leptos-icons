#[cfg(feature = "IoBagCheck")]
use leptos::*;
#[cfg(feature = "IoBagCheck")]
///This icon requires the feature `IoBagCheck` to be enabled.
#[component]
pub fn BagCheck(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M454.65,169.4A31.82,31.82,0,0,0,432,160H368V144a112,112,0,0,0-224,0v16H80a32,32,0,0,0-32,32V408c0,39,33,72,72,72H392a72.22,72.22,0,0,0,50.48-20.55A69.48,69.48,0,0,0,464,409.25V192A31.75,31.75,0,0,0,454.65,169.4ZM332.49,274l-89.6,112a16,16,0,0,1-12.23,6h-.26a16,16,0,0,1-12.16-5.6l-38.4-44.88a16,16,0,1,1,24.32-20.8L230,350.91,307.51,254a16,16,0,0,1,25,20ZM336,160H176V144a80,80,0,0,1,160,0Z"
        /> < title > { title } < / title > < / svg >
    }
}
