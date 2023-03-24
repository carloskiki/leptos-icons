#[cfg(feature = "RiLogosFillDrive")]
use leptos::*;
#[cfg(feature = "RiLogosFillDrive")]
///This icon requires the feature `RiLogosFillDrive` to be enabled.
#[component]
pub fn Drive(
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
        "none" d = "M0 0h24v24H0z" />< path d =
        "M7.94 4.146l3.482 6.03-5.94 10.293L2 14.44 7.94 4.146zm2.176 10.294H22l-3.482 6.029H6.635l3.481-6.029zm4.343-1L8.518 3.145h6.964l5.94 10.295H14.46z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
