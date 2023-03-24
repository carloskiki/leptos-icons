#[cfg(feature = "IoDiamondOutline")]
use leptos::*;
#[cfg(feature = "IoDiamondOutline")]
///This icon requires the feature `IoDiamondOutline` to be enabled.
#[component]
pub fn DiamondOutline(
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
        stroke_witdh = "0" style = style id = "icons" viewBox = "0 0 512 512" width =
        size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M35.42,188.21,243.17,457.67a16.17,16.17,0,0,0,25.66,0L476.58,188.21a16.52,16.52,0,0,0,.95-18.75L407.06,55.71A16.22,16.22,0,0,0,393.27,48H118.73a16.22,16.22,0,0,0-13.79,7.71L34.47,169.46A16.52,16.52,0,0,0,35.42,188.21Z"
        fill = "none" stroke = "#000" stroke - linecap = "round" stroke - linejoin =
        "round" stroke - width = "32" />< line xmlns = "http://www.w3.org/2000/svg" x1 =
        "48" y1 = "176" x2 = "464" y2 = "176" fill = "none" stroke = "#000" stroke -
        linecap = "round" stroke - linejoin = "round" stroke - width = "32" />< polyline
        xmlns = "http://www.w3.org/2000/svg" points = "400 64 352 176 256 48" fill =
        "none" stroke = "#000" stroke - linecap = "round" stroke - linejoin = "round"
        stroke - width = "32" />< polyline xmlns = "http://www.w3.org/2000/svg" points =
        "112 64 160 176 256 48" fill = "none" stroke = "#000" stroke - linecap = "round"
        stroke - linejoin = "round" stroke - width = "32" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "256" y1 = "448" x2 = "160" y2 = "176" fill =
        "none" stroke = "#000" stroke - linecap = "round" stroke - linejoin = "round"
        stroke - width = "32" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "256" y1
        = "448" x2 = "352" y2 = "176" fill = "none" stroke = "#000" stroke - linecap =
        "round" stroke - linejoin = "round" stroke - width = "32" /> < title > { title }
        < / title > < / svg >
    }
}
