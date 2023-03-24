#[cfg(feature = "IoMicOffSharp")]
use leptos::*;
#[cfg(feature = "IoMicOffSharp")]
///This icon requires the feature `IoMicOffSharp` to be enabled.
#[component]
pub fn MicOffSharp(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < line xmlns =
        "http://www.w3.org/2000/svg" x1 = "432" y1 = "400" x2 = "96" y2 = "64" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M368,192v48a111.74,111.74,0,0,1-2.93,25.45L390.65,291A143.07,143.07,0,0,0,400,240V192Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M272,432V383.11a143.11,143.11,0,0,0,56.65-18.83L305,340.65A112.13,112.13,0,0,1,144,240V192H112v48c0,74,56.1,135.12,128,143.11V432H176v32H336V432Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M336,236.37V128c0-44.86-35.14-80-80-80a79.68,79.68,0,0,0-69,39.34" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M176,211.63V239a80.89,80.89,0,0,0,23.45,56.9,78.55,78.55,0,0,0,81,20.21Z" /> <
        title > { title } < / title > < / svg >
    }
}
