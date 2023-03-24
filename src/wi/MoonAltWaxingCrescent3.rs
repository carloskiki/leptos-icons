#[cfg(feature = "WiMoonAltWaxingCrescent3")]
use leptos::*;
#[cfg(feature = "WiMoonAltWaxingCrescent3")]
///This icon requires the feature `WiMoonAltWaxingCrescent3` to be enabled.
#[component]
pub fn MoonAltWaxingCrescent3(
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
        "M3.74,14.44c0-1.52,0.3-2.98,0.89-4.37s1.4-2.58,2.4-3.59s2.2-1.81,3.59-2.4s2.84-0.89,4.37-0.89s2.98,0.3,4.37,0.89&#xA;	s2.59,1.4,3.6,2.4s1.81,2.2,2.4,3.59s0.89,2.84,0.89,4.37s-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.6s-2.2,1.81-3.6,2.4&#xA;	s-2.85,0.89-4.37,0.89s-2.98-0.3-4.37-0.89s-2.58-1.4-3.59-2.4s-1.81-2.2-2.4-3.6S3.74,15.97,3.74,14.44z M15.58,4.41&#xA;	c1.28,1.1,2.32,2.51,3.13,4.22s1.22,3.65,1.22,5.82c0,4.64-1.39,7.99-4.16,10.05c1.28-0.1,2.49-0.43,3.63-1s2.13-1.29,2.96-2.18&#xA;	s1.49-1.93,1.97-3.13s0.73-2.44,0.73-3.74c0-1.75-0.42-3.38-1.26-4.89s-1.99-2.72-3.44-3.64S17.31,4.5,15.58,4.41z"
        /> < title > { title } < / title > < / svg >
    }
}
