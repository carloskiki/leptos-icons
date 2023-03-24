#[cfg(feature = "WiMoonAltWaningCrescent3")]
use leptos::*;
#[cfg(feature = "WiMoonAltWaningCrescent3")]
///This icon requires the feature `WiMoonAltWaningCrescent3` to be enabled.
#[component]
pub fn MoonAltWaningCrescent3(
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
        "M3.74,14.44c0-1.52,0.3-2.98,0.89-4.37s1.4-2.58,2.4-3.59s2.2-1.81,3.59-2.4s2.84-0.89,4.37-0.89s2.98,0.3,4.37,0.89&#xA;	s2.59,1.4,3.6,2.4s1.81,2.2,2.4,3.59s0.89,2.84,0.89,4.37s-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.6s-2.2,1.81-3.6,2.4&#xA;	s-2.85,0.89-4.37,0.89s-2.98-0.3-4.37-0.89s-2.58-1.4-3.59-2.4s-1.81-2.2-2.4-3.6S3.74,15.97,3.74,14.44z M4.94,14.44&#xA;	c0,1.34,0.26,2.62,0.77,3.85s1.21,2.29,2.08,3.19s1.92,1.62,3.13,2.16s2.48,0.83,3.81,0.87c-1.71-2.32-2.56-5.68-2.56-10.06&#xA;	c0-1.87,0.23-3.67,0.69-5.41s1.11-3.29,1.95-4.64c-1.8,0.03-3.45,0.5-4.96,1.41s-2.7,2.13-3.58,3.65S4.94,12.65,4.94,14.44z"
        /> < title > { title } < / title > < / svg >
    }
}
