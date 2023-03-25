#[cfg(feature = "IoDiceSharp")]
use leptos::*;
#[cfg(feature = "IoDiceSharp")]
///This icon requires the feature `IoDiceSharp` to be enabled.
#[component]
pub fn DiceSharp(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M48,366.92,240,480V284L48,170ZM192,288c8.84,0,16,10.75,16,24s-7.16,24-16,24-16-10.75-16-24S183.16,288,192,288ZM96,320c8.84,0,16,10.75,16,24s-7.16,24-16,24-16-10.75-16-24S87.16,320,96,320Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M272,284V480L464,366.92V170Zm48,140c-8.84,0-16-10.75-16-24s7.16-24,16-24,16,10.75,16,24S328.84,424,320,424Zm0-88c-8.84,0-16-10.75-16-24s7.16-24,16-24,16,10.75,16,24S328.84,336,320,336Zm96,32c-8.84,0-16-10.75-16-24s7.16-24,16-24,16,10.75,16,24S424.84,368,416,368Zm0-88c-8.84,0-16-10.75-16-24s7.16-24,16-24,16,10.75,16,24S424.84,280,416,280Zm32,77.64h0Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M256,32,64,144,256,256,448,144Zm0,120c-13.25,0-24-7.16-24-16s10.75-16,24-16,24,7.16,24,16S269.25,152,256,152Z"
        /> < title > { title } < / title > < / svg >
    }
}
