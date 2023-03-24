#[cfg(feature = "IoVolumeHighSharp")]
use leptos::*;
#[cfg(feature = "IoVolumeHighSharp")]
///This icon requires the feature `IoVolumeHighSharp` to be enabled.
#[component]
pub fn VolumeHighSharp(
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
        "0 0 512 512" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M320,320c9.74-19.38,16-40.84,16-64,0-23.48-6-44.42-16-64" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M368,368c19.48-33.92,32-64.06,32-112s-12-77.74-32-112" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M416,416c30-46,48-91.43,48-160S446,143,416,96" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:32px"
        />< polygon xmlns = "http://www.w3.org/2000/svg" points =
        "125.65 176.1 32 176.1 32 335.9 125.65 335.9 256 440 256 72 125.65 176.1" /> <
        title > { title } < / title > < / svg >
    }
}
