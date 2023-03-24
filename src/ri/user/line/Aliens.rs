#[cfg(feature = "RiUserLineAliens")]
use leptos::*;
#[cfg(feature = "RiUserLineAliens")]
///This icon requires the feature `RiUserLineAliens` to be enabled.
#[component]
pub fn Aliens(
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
        "none" d = "M0 0h24v24H0z" />< path fill - rule = "nonzero" d =
        "M12 2a8.5 8.5 0 0 1 8.5 8.5c0 6.5-5.5 12-8.5 12s-8.5-5.5-8.5-12A8.5 8.5 0 0 1 12 2zm0 2a6.5 6.5 0 0 0-6.5 6.5c0 4.794 4.165 10 6.5 10s6.5-5.206 6.5-10A6.5 6.5 0 0 0 12 4zm5.5 7c.16 0 .319.008.475.025a4.5 4.5 0 0 1-4.95 4.95A4.5 4.5 0 0 1 17.5 11zm-11 0a4.5 4.5 0 0 1 4.475 4.975 4.5 4.5 0 0 1-4.95-4.95C6.18 11.008 6.34 11 6.5 11z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
