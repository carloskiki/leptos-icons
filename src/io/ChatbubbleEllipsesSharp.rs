#[cfg(feature = "IoChatbubbleEllipsesSharp")]
use leptos::*;
#[cfg(feature = "IoChatbubbleEllipsesSharp")]
///This icon requires the feature `IoChatbubbleEllipsesSharp` to be enabled.
#[component]
pub fn ChatbubbleEllipsesSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M475.22,206.52C464.88,157.87,437.46,113.59,398,81.84A227.4,227.4,0,0,0,255.82,32C194.9,32,138,55.47,95.46,98.09,54.35,139.33,31.82,193.78,32,251.37A215.66,215.66,0,0,0,67.65,370.13L72,376.18,48,480l114.8-28.56s2.3.77,4,1.42,16.33,6.26,31.85,10.6c12.9,3.6,39.74,9,60.77,9,59.65,0,115.35-23.1,156.83-65.06C457.36,365.77,480,310.42,480,251.49A213.5,213.5,0,0,0,475.22,206.52ZM160,288a32,32,0,1,1,32-32A32,32,0,0,1,160,288Zm96,0a32,32,0,1,1,32-32A32,32,0,0,1,256,288Zm96,0a32,32,0,1,1,32-32A32,32,0,0,1,352,288Z"
        /> < title > { title } < / title > < / svg >
    }
}
