#[cfg(feature = "IoImagesOutline")]
use leptos::*;
#[cfg(feature = "IoImagesOutline")]
///This icon requires the feature `IoImagesOutline` to be enabled.
#[component]
pub fn ImagesOutline(
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
        "M432,112V96a48.14,48.14,0,0,0-48-48H64A48.14,48.14,0,0,0,16,96V352a48.14,48.14,0,0,0,48,48H80"
        style = "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" />< rect
        xmlns = "http://www.w3.org/2000/svg" x = "96" y = "128" width = "400" height =
        "336" rx = "45.99" ry = "45.99" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" />< ellipse xmlns
        = "http://www.w3.org/2000/svg" cx = "372.92" cy = "219.64" rx = "30.77" ry =
        "30.55" style = "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M342.15,372.17,255,285.78a30.93,30.93,0,0,0-42.18-1.21L96,387.64" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M265.23,464,383.82,346.27a31,31,0,0,1,41.46-1.87L496,402.91" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        /> < title > { title } < / title > < / svg >
    }
}
