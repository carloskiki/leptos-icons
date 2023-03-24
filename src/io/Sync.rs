#[cfg(feature = "IoSync")]
use leptos::*;
#[cfg(feature = "IoSync")]
///This icon requires the feature `IoSync` to be enabled.
#[component]
pub fn Sync(
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
        "M434.67,285.59v-29.8C434.67,157.06,354.43,77,255.47,77a179,179,0,0,0-140.14,67.36m-38.53,82v29.8C76.8,355,157,435,256,435a180.45,180.45,0,0,0,140-66.92"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< polyline xmlns = "http://www.w3.org/2000/svg" points =
        "32 256 76 212 122 256" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< polyline xmlns = "http://www.w3.org/2000/svg" points =
        "480 256 436 300 390 256" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        /> < title > { title } < / title > < / svg >
    }
}
