#[cfg(feature = "IoRibbonSharp")]
use leptos::*;
#[cfg(feature = "IoRibbonSharp")]
///This icon requires the feature `IoRibbonSharp` to be enabled.
#[component]
pub fn RibbonSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M256,336c-5.22,0-10.4-.24-15.51-.69A176.12,176.12,0,0,1,109.2,256.94L20,416H135l58,96,82.53-177.09A177.53,177.53,0,0,1,256,336Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M403,256.74a176.9,176.9,0,0,1-88.18,69.14L273.7,415.5,319,512l58-96H492Z" /><
        circle xmlns = "http://www.w3.org/2000/svg" cx = "256.02" cy = "160" r = "48" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M256,16c-79.4,0-144,64.6-144,144s64.6,144,144,144,144-64.6,144-144S335.4,16,256,16Zm0,224a80,80,0,1,1,80-80A80.09,80.09,0,0,1,256,240Z"
        /> < title > { title } < / title > < / svg >
    }
}
