#[cfg(feature = "RiLogosFillOpenSource")]
use leptos::*;
#[cfg(feature = "RiLogosFillOpenSource")]
///This icon requires the feature `RiLogosFillOpenSource` to be enabled.
#[component]
pub fn OpenSource(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = { size.clone() }
        height = { size } > < g xmlns = "http://www.w3.org/2000/svg" >< path fill =
        "none" d = "M0 0H24V24H0z" />< path d =
        "M12 2c5.523 0 10 4.477 10 10 0 4.13-2.504 7.676-6.077 9.201l-2.518-6.55C14.354 14.148 15 13.15 15 12c0-1.657-1.343-3-3-3s-3 1.343-3 3c0 1.15.647 2.148 1.596 2.652l-2.518 6.55C4.504 19.675 2 16.13 2 12 2 6.477 6.477 2 12 2z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
