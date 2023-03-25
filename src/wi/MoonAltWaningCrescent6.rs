#[cfg(feature = "WiMoonAltWaningCrescent6")]
use leptos::*;
#[cfg(feature = "WiMoonAltWaningCrescent6")]
///This icon requires the feature `WiMoonAltWaningCrescent6` to be enabled.
#[component]
pub fn MoonAltWaningCrescent6(
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
        "M3.74,14.44c0-1.52,0.3-2.98,0.89-4.37s1.4-2.58,2.4-3.59s2.2-1.81,3.59-2.4s2.84-0.89,4.37-0.89s2.98,0.3,4.37,0.89&#xA;	s2.59,1.4,3.6,2.4s1.81,2.2,2.4,3.59s0.89,2.84,0.89,4.37s-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.6s-2.2,1.81-3.6,2.4&#xA;	s-2.85,0.89-4.37,0.89s-2.98-0.3-4.37-0.89s-2.58-1.4-3.59-2.4s-1.81-2.2-2.4-3.6S3.74,15.97,3.74,14.44z M4.94,14.44&#xA;	c0,2.48,0.8,4.66,2.41,6.53s3.62,3.01,6.03,3.41c-1.01-0.5-1.86-1.1-2.56-1.82s-1.25-1.5-1.63-2.37s-0.66-1.77-0.83-2.7&#xA;	s-0.26-1.95-0.26-3.06c0-2.11,0.5-4.06,1.49-5.84s2.37-3.16,4.12-4.12c-1.63,0.21-3.11,0.77-4.45,1.7S6.87,8.3,6.1,9.76&#xA;	S4.94,12.77,4.94,14.44z"
        /> < title > { title } < / title > < / svg >
    }
}
