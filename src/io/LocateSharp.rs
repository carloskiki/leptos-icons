#[cfg(feature = "IoLocateSharp")]
use leptos::*;
#[cfg(feature = "IoLocateSharp")]
///This icon requires the feature `IoLocateSharp` to be enabled.
#[component]
pub fn LocateSharp(
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
        "http://www.w3.org/2000/svg" x1 = "256" y1 = "96" x2 = "256" y2 = "56" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-linejoin:round;stroke-width:48px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "256" y1 = "456" x2 = "256" y2
        = "416" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-linejoin:round;stroke-width:48px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M256,112A144,144,0,1,0,400,256,144,144,0,0,0,256,112Z" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-linejoin:round;stroke-width:48px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "416" y1 = "256" x2 = "456" y2
        = "256" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-linejoin:round;stroke-width:48px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "56" y1 = "256" x2 = "96" y2 =
        "256" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-linejoin:round;stroke-width:48px"
        /> < title > { title } < / title > < / svg >
    }
}
