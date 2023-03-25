#[cfg(feature = "WiMoonWaxingGibbous3")]
use leptos::*;
#[cfg(feature = "WiMoonWaxingGibbous3")]
///This icon requires the feature `WiMoonWaxingGibbous3` to be enabled.
#[component]
pub fn MoonWaxingGibbous3(
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
        "M11.8,14.43c0,2.39,0.24,4.52,0.71,6.39s1.31,3.5,2.51,4.89c1.52,0,2.98-0.3,4.37-0.89s2.59-1.4,3.6-2.4s1.81-2.2,2.4-3.6&#xA;	s0.89-2.85,0.89-4.39s-0.3-2.99-0.89-4.38s-1.4-2.58-2.4-3.59s-2.2-1.81-3.6-2.4s-2.85-0.89-4.37-0.89&#xA;	c-1.02,1.46-1.81,3.16-2.37,5.13S11.8,12.3,11.8,14.43z"
        /> < title > { title } < / title > < / svg >
    }
}
