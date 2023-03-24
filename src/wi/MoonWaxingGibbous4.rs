#[cfg(feature = "WiMoonWaxingGibbous4")]
use leptos::*;
#[cfg(feature = "WiMoonWaxingGibbous4")]
///This icon requires the feature `WiMoonWaxingGibbous4` to be enabled.
#[component]
pub fn MoonWaxingGibbous4(
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
        "M10.73,14.43c0,1.19,0.07,2.29,0.2,3.3s0.35,2,0.67,2.99s0.76,1.9,1.33,2.75s1.27,1.59,2.09,2.25c1.53,0,2.99-0.3,4.38-0.89&#xA;	s2.58-1.4,3.59-2.4s1.81-2.2,2.4-3.6s0.89-2.85,0.89-4.39c0-2.04-0.5-3.93-1.51-5.65s-2.37-3.1-4.1-4.1s-3.61-1.51-5.65-1.51&#xA;	c-1.35,1.3-2.4,2.94-3.16,4.93S10.73,12.19,10.73,14.43z"
        /> < title > { title } < / title > < / svg >
    }
}
