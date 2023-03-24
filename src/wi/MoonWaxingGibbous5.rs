#[cfg(feature = "WiMoonWaxingGibbous5")]
use leptos::*;
#[cfg(feature = "WiMoonWaxingGibbous5")]
///This icon requires the feature `WiMoonWaxingGibbous5` to be enabled.
#[component]
pub fn MoonWaxingGibbous5(
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
        stroke_witdh = "0" style = style version = "1.1" id = "Layer_1" x = "0px" y =
        "0px" viewBox = "0 0 30 30" style = "enable-background:new 0 0 30 30;" space =
        "preserve" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" d =
        "M9.65,14.43c0,1.24,0.08,2.38,0.25,3.41s0.44,2.05,0.83,3.04s0.95,1.89,1.67,2.71s1.6,1.52,2.62,2.12&#xA;	c1.52,0,2.98-0.3,4.37-0.89s2.59-1.4,3.6-2.4s1.81-2.2,2.4-3.6s0.89-2.85,0.89-4.39s-0.3-2.99-0.89-4.38s-1.4-2.58-2.4-3.59&#xA;	s-2.2-1.81-3.6-2.4s-2.85-0.89-4.37-0.89c-1.67,1.14-2.98,2.72-3.94,4.74S9.65,12.09,9.65,14.43z"
        /> < title > { title } < / title > < / svg >
    }
}
