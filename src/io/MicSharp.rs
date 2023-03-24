#[cfg(feature = "IoMicSharp")]
use leptos::*;
#[cfg(feature = "IoMicSharp")]
///This icon requires the feature `IoMicSharp` to be enabled.
#[component]
pub fn MicSharp(
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
        "http://www.w3.org/2000/svg" > < line xmlns = "http://www.w3.org/2000/svg" x1 =
        "192" y1 = "448" x2 = "320" y2 = "448" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M384,208v32c0,70.4-57.6,128-128,128h0c-70.4,0-128-57.6-128-128V208" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "256" y1 = "368" x2 = "256" y2
        = "448" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M256,320a78.83,78.83,0,0,1-56.55-24.1A80.89,80.89,0,0,1,176,239V128a79.69,79.69,0,0,1,80-80c44.86,0,80,35.14,80,80V239C336,283.66,300.11,320,256,320Z"
        /> < title > { title } < / title > < / svg >
    }
}
