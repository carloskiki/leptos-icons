#[cfg(feature = "WiMoonAltWaningCrescent4")]
use leptos::*;
#[cfg(feature = "WiMoonAltWaningCrescent4")]
///This icon requires the feature `WiMoonAltWaningCrescent4` to be enabled.
#[component]
pub fn MoonAltWaningCrescent4(
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
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style version = "1.1" id = "Layer_1" x = "0px" y =
        "0px" viewBox = "0 0 30 30" style = "enable-background:new 0 0 30 30;" space =
        "preserve" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" d =
        "M3.74,14.44c0-1.52,0.3-2.98,0.89-4.37s1.4-2.58,2.4-3.59s2.2-1.81,3.59-2.4s2.84-0.89,4.37-0.89s2.98,0.3,4.37,0.89&#xA;	s2.59,1.4,3.6,2.4s1.81,2.2,2.4,3.59s0.89,2.84,0.89,4.37s-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.6s-2.2,1.81-3.6,2.4&#xA;	s-2.85,0.89-4.37,0.89s-2.98-0.3-4.37-0.89s-2.58-1.4-3.59-2.4s-1.81-2.2-2.4-3.6S3.74,15.97,3.74,14.44z M4.94,14.44&#xA;	c0,1.33,0.25,2.6,0.75,3.81s1.18,2.26,2.03,3.15s1.87,1.61,3.05,2.17s2.43,0.87,3.74,0.94c-1.24-1.19-2.11-2.63-2.61-4.31&#xA;	s-0.75-3.6-0.75-5.76c0-1.93,0.31-3.78,0.92-5.54s1.47-3.26,2.56-4.5c-1.77,0.07-3.39,0.56-4.88,1.47S7.09,8,6.23,9.51&#xA;	S4.94,12.68,4.94,14.44z"
        /> < title > { title } < / title > < / svg >
    }
}
