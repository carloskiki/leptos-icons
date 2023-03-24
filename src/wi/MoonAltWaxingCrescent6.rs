#[cfg(feature = "WiMoonAltWaxingCrescent6")]
use leptos::*;
#[cfg(feature = "WiMoonAltWaxingCrescent6")]
///This icon requires the feature `WiMoonAltWaxingCrescent6` to be enabled.
#[component]
pub fn MoonAltWaxingCrescent6(
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
        "M3.74,14.44c0-1.52,0.3-2.98,0.89-4.37s1.4-2.58,2.4-3.59s2.2-1.81,3.59-2.4s2.84-0.89,4.37-0.89s2.98,0.3,4.37,0.89&#xA;	s2.59,1.4,3.6,2.4s1.81,2.2,2.4,3.59s0.89,2.84,0.89,4.37s-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.6s-2.2,1.81-3.6,2.4&#xA;	s-2.85,0.89-4.37,0.89s-2.98-0.3-4.37-0.89s-2.58-1.4-3.59-2.4s-1.81-2.2-2.4-3.6S3.74,15.97,3.74,14.44z M15.01,4.39&#xA;	c1.23,3.54,1.85,6.89,1.85,10.05c0,3.93-0.59,7.28-1.77,10.06c1.35-0.01,2.64-0.28,3.87-0.81s2.3-1.25,3.19-2.15s1.6-1.97,2.12-3.21&#xA;	s0.79-2.54,0.79-3.9s-0.27-2.66-0.8-3.9s-1.25-2.31-2.15-3.21s-1.97-1.61-3.21-2.15S16.36,4.39,15.01,4.39z"
        /> < title > { title } < / title > < / svg >
    }
}
