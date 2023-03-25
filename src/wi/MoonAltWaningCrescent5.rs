#[cfg(feature = "WiMoonAltWaningCrescent5")]
use leptos::*;
#[cfg(feature = "WiMoonAltWaningCrescent5")]
///This icon requires the feature `WiMoonAltWaningCrescent5` to be enabled.
#[component]
pub fn MoonAltWaningCrescent5(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    let style = format!("{} color: {};", style, color);
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style version = "1.1" id = "Layer_1" x = "0px" y =
        "0px" viewBox = "0 0 30 30" style = "enable-background:new 0 0 30 30;" space =
        "preserve" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" d =
        "M3.74,14.44c0-1.52,0.3-2.98,0.89-4.37s1.4-2.58,2.4-3.59s2.2-1.81,3.59-2.4s2.84-0.89,4.37-0.89s2.98,0.3,4.37,0.89&#xA;	s2.59,1.4,3.6,2.4s1.81,2.2,2.4,3.59s0.89,2.84,0.89,4.37s-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.6s-2.2,1.81-3.6,2.4&#xA;	s-2.85,0.89-4.37,0.89s-2.98-0.3-4.37-0.89s-2.58-1.4-3.59-2.4s-1.81-2.2-2.4-3.6S3.74,15.97,3.74,14.44z M4.94,14.44&#xA;	c0,1.27,0.23,2.49,0.7,3.66s1.09,2.2,1.89,3.08s1.75,1.61,2.85,2.19s2.28,0.94,3.52,1.08c-1.75-1.04-2.98-2.39-3.7-4.07&#xA;	s-1.08-3.66-1.08-5.93c0-2.07,0.44-4,1.32-5.78s2.1-3.2,3.66-4.24c-1.26,0.11-2.46,0.45-3.59,1.02s-2.1,1.3-2.92,2.19&#xA;	s-1.46,1.92-1.93,3.11S4.94,13.15,4.94,14.44z"
        /> < title > { title } < / title > < / svg >
    }
}
