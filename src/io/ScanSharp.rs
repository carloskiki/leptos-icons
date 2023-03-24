#[cfg(feature = "IoScanSharp")]
use leptos::*;
#[cfg(feature = "IoScanSharp")]
///This icon requires the feature `IoScanSharp` to be enabled.
#[component]
pub fn ScanSharp(
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
        "M388,466H320V422h68a34,34,0,0,0,34-34V320h44v68A78.09,78.09,0,0,1,388,466Z" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M466,192H422V124a34,34,0,0,0-34-34H320V46h68a78.09,78.09,0,0,1,78,78Z" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M192,466H124a78.09,78.09,0,0,1-78-78V320H90v68a34,34,0,0,0,34,34h68Z" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M90,192H46V124a78.09,78.09,0,0,1,78-78h68V90H124a34,34,0,0,0-34,34Z" /> < title
        > { title } < / title > < / svg >
    }
}
