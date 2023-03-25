#[cfg(feature = "IoLayersOutline")]
use leptos::*;
#[cfg(feature = "IoLayersOutline")]
///This icon requires the feature `IoLayersOutline` to be enabled.
#[component]
pub fn LayersOutline(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M434.8,137.65,285.44,69.55c-16.19-7.4-42.69-7.4-58.88,0L77.3,137.65c-17.6,8-17.6,21.09,0,29.09l148,67.5c16.89,7.7,44.69,7.7,61.58,0l148-67.5C452.4,158.74,452.4,145.64,434.8,137.65Z"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M160,308.52,77.3,345.63c-17.6,8-17.6,21.1,0,29.1l148,67.5c16.89,7.69,44.69,7.69,61.58,0l148-67.5c17.6-8,17.6-21.1,0-29.1l-79.94-38.47"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M160,204.48,77.2,241.64c-17.6,8-17.6,21.1,0,29.1l148,67.49c16.89,7.7,44.69,7.7,61.58,0l148-67.49c17.7-8,17.7-21.1.1-29.1L352,204.48"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        /> < title > { title } < / title > < / svg >
    }
}
