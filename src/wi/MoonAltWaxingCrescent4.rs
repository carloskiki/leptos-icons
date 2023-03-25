#[cfg(feature = "WiMoonAltWaxingCrescent4")]
use leptos::*;
#[cfg(feature = "WiMoonAltWaxingCrescent4")]
///This icon requires the feature `WiMoonAltWaxingCrescent4` to be enabled.
#[component]
pub fn MoonAltWaxingCrescent4(
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
        "M3.75,14.44c0-1.52,0.3-2.98,0.89-4.37s1.4-2.58,2.4-3.59s2.2-1.81,3.59-2.4S13.48,3.19,15,3.19s2.98,0.3,4.37,0.89&#xA;	s2.59,1.4,3.6,2.4s1.81,2.2,2.4,3.59s0.89,2.84,0.89,4.37s-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.6s-2.2,1.81-3.6,2.4&#xA;	s-2.85,0.89-4.37,0.89s-2.98-0.3-4.37-0.89s-2.58-1.4-3.59-2.4s-1.81-2.2-2.4-3.6S3.75,15.97,3.75,14.44z M15.35,4.39&#xA;	c1.05,1.27,1.91,2.75,2.57,4.44s0.99,3.56,0.99,5.61c0,4.39-1.14,7.75-3.43,10.06c1.31-0.06,2.55-0.37,3.74-0.92s2.2-1.28,3.05-2.18&#xA;	s1.53-1.95,2.04-3.16s0.75-2.48,0.75-3.81c0-1.78-0.43-3.43-1.3-4.94s-2.04-2.73-3.53-3.65S17.12,4.45,15.35,4.39z"
        /> < title > { title } < / title > < / svg >
    }
}
