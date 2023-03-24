#[cfg(feature = "IoShuffleSharp")]
use leptos::*;
#[cfg(feature = "IoShuffleSharp")]
///This icon requires the feature `IoShuffleSharp` to be enabled.
#[component]
pub fn ShuffleSharp(
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
        "0 0 512 512" width = { size.clone() } height = { size } > < polyline xmlns =
        "http://www.w3.org/2000/svg" points = "400 304 448 352 400 400" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:32px"
        />< polyline xmlns = "http://www.w3.org/2000/svg" points =
        "400 112 448 160 400 208" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:32px"
        />< polyline xmlns = "http://www.w3.org/2000/svg" points =
        "64 352 192 352 252 260" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:32px"
        />< polyline xmlns = "http://www.w3.org/2000/svg" points =
        "64 160 192 160 320 352 416 352" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:32px"
        />< polyline xmlns = "http://www.w3.org/2000/svg" points =
        "416 160 320 160 288 208" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:32px"
        /> < title > { title } < / title > < / svg >
    }
}
