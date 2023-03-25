#[cfg(feature = "WiMoonWaxingCrescent4")]
use leptos::*;
#[cfg(feature = "WiMoonWaxingCrescent4")]
///This icon requires the feature `WiMoonWaxingCrescent4` to be enabled.
#[component]
pub fn MoonWaxingCrescent4(
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
        "M15.01,25.71c2.04,0,3.92-0.5,5.65-1.51s3.09-2.37,4.09-4.1s1.51-3.61,1.51-5.65s-0.5-3.92-1.51-5.65s-2.37-3.09-4.09-4.09&#xA;	s-3.61-1.51-5.65-1.51c1.29,1.39,2.24,3.07,2.84,5.05s0.91,4.05,0.91,6.2c0,0.88-0.03,1.69-0.08,2.44s-0.16,1.55-0.32,2.41&#xA;	s-0.38,1.65-0.64,2.37s-0.63,1.43-1.09,2.15S15.62,25.15,15.01,25.71z"
        /> < title > { title } < / title > < / svg >
    }
}
