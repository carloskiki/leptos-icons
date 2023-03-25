#[cfg(feature = "WiMoonWaxingCrescent3")]
use leptos::*;
#[cfg(feature = "WiMoonWaxingCrescent3")]
///This icon requires the feature `WiMoonWaxingCrescent3` to be enabled.
#[component]
pub fn MoonWaxingCrescent3(
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
        "M15.01,25.71c2.04,0,3.92-0.5,5.65-1.51s3.09-2.37,4.09-4.1s1.51-3.61,1.51-5.65s-0.5-3.92-1.51-5.65s-2.37-3.09-4.09-4.09&#xA;	s-3.61-1.51-5.65-1.51c1.71,1.26,2.97,2.9,3.78,4.91S20,12.24,20,14.44c0,0.9-0.03,1.73-0.1,2.5s-0.21,1.59-0.43,2.47&#xA;	s-0.51,1.68-0.86,2.4s-0.83,1.42-1.45,2.12S15.83,25.21,15.01,25.71z"
        /> < title > { title } < / title > < / svg >
    }
}
