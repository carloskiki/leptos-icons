#[cfg(feature = "IoShirtOutline")]
use leptos::*;
#[cfg(feature = "IoShirtOutline")]
///This icon requires the feature `IoShirtOutline` to be enabled.
#[component]
pub fn ShirtOutline(
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
        "M314.56,48S291.78,56,256,56s-58.56-8-58.56-8a31.94,31.94,0,0,0-10.57,1.8L32,104l16.63,88,48.88,5.52A24,24,0,0,1,118.8,222.1L112,464H400l-6.8-241.9a24,24,0,0,1,21.29-24.58L463.37,192,480,104,325.13,49.8A31.94,31.94,0,0,0,314.56,48Z"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M333.31,52.66a80,80,0,0,1-154.62,0" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        /> < title > { title } < / title > < / svg >
    }
}
