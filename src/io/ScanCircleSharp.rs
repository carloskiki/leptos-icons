#[cfg(feature = "IoScanCircleSharp")]
use leptos::*;
#[cfg(feature = "IoScanCircleSharp")]
///This icon requires the feature `IoScanCircleSharp` to be enabled.
#[component]
pub fn ScanCircleSharp(
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
        "M256,48C141.31,48,48,141.31,48,256s93.31,208,208,208,208-93.31,208-208S370.69,48,256,48ZM232,368H188a44.05,44.05,0,0,1-44-44V280h32v44a12,12,0,0,0,12,12h44Zm0-192H188a12,12,0,0,0-12,12v44H144V188a44.05,44.05,0,0,1,44-44h44ZM368,324a44.05,44.05,0,0,1-44,44H280V336h44a12,12,0,0,0,12-12V280h32Zm0-92H336V188a12,12,0,0,0-12-12H280V144h44a44.05,44.05,0,0,1,44,44Z"
        /> < title > { title } < / title > < / svg >
    }
}
