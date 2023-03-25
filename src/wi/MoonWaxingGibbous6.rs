#[cfg(feature = "WiMoonWaxingGibbous6")]
use leptos::*;
#[cfg(feature = "WiMoonWaxingGibbous6")]
///This icon requires the feature `WiMoonWaxingGibbous6` to be enabled.
#[component]
pub fn MoonWaxingGibbous6(
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
        "M8.58,14.43c0,1.03,0.06,1.97,0.18,2.83s0.32,1.73,0.62,2.59s0.69,1.65,1.16,2.34s1.1,1.35,1.85,1.96s1.63,1.12,2.63,1.55&#xA;	c1.53,0,2.99-0.3,4.38-0.89s2.58-1.4,3.59-2.4s1.81-2.2,2.4-3.6s0.89-2.85,0.89-4.39c0-2.04-0.5-3.93-1.51-5.65s-2.37-3.1-4.1-4.1&#xA;	s-3.61-1.51-5.65-1.51c-1.99,1-3.56,2.51-4.72,4.55S8.58,11.99,8.58,14.43z"
        /> < title > { title } < / title > < / svg >
    }
}
