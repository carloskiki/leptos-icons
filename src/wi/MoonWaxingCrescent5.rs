#[cfg(feature = "WiMoonWaxingCrescent5")]
use leptos::*;
#[cfg(feature = "WiMoonWaxingCrescent5")]
///This icon requires the feature `WiMoonWaxingCrescent5` to be enabled.
#[component]
pub fn MoonWaxingCrescent5(
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
        "M14.99,25.71c2.04,0,3.93-0.5,5.65-1.51s3.1-2.37,4.1-4.1s1.51-3.61,1.51-5.65s-0.5-3.92-1.51-5.65s-2.37-3.09-4.1-4.09&#xA;	s-3.61-1.51-5.65-1.51c1.67,2.9,2.5,6.65,2.5,11.25c0,2.33-0.17,4.43-0.52,6.3S15.97,24.26,14.99,25.71z"
        /> < title > { title } < / title > < / svg >
    }
}
