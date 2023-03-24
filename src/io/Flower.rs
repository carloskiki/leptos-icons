#[cfg(feature = "IoFlower")]
use leptos::*;
#[cfg(feature = "IoFlower")]
///This icon requires the feature `IoFlower` to be enabled.
#[component]
pub fn Flower(
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
        "http://www.w3.org/2000/svg" > < circle xmlns = "http://www.w3.org/2000/svg" cx =
        "256" cy = "256" r = "48" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M475.93,303.91a67.49,67.49,0,0,0-44.34-115.53,5.2,5.2,0,0,1-4.58-3.21h0a5.21,5.21,0,0,1,1-5.51A67.83,67.83,0,0,0,378,66.33h-.25A67.13,67.13,0,0,0,332.35,84a5.21,5.21,0,0,1-5.52,1h0a5.23,5.23,0,0,1-3.22-4.58,67.68,67.68,0,0,0-135.23,0A5.2,5.2,0,0,1,185.17,85h0a5.21,5.21,0,0,1-5.52-1,67.11,67.11,0,0,0-45.44-17.69H134A67.91,67.91,0,0,0,84,179.65a5.21,5.21,0,0,1,1,5.51h0a5.2,5.2,0,0,1-4.58,3.21,67.71,67.71,0,0,0,0,135.23A5.23,5.23,0,0,1,85,326.83h0a5.22,5.22,0,0,1-1,5.52,67.54,67.54,0,0,0,50.08,113h.25A67.38,67.38,0,0,0,179.65,428a5.21,5.21,0,0,1,5.51-1h0a5.2,5.2,0,0,1,3.21,4.58,67.71,67.71,0,0,0,135.23,0,5.23,5.23,0,0,1,3.22-4.58h0a5.21,5.21,0,0,1,5.51,1,67.38,67.38,0,0,0,45.29,17.42h.25a67.48,67.48,0,0,0,50.08-113,5.22,5.22,0,0,1-1-5.52h0a5.23,5.23,0,0,1,4.58-3.22A67.31,67.31,0,0,0,475.93,303.91ZM256,336a80,80,0,1,1,80-80A80.09,80.09,0,0,1,256,336Z"
        /> < title > { title } < / title > < / svg >
    }
}
