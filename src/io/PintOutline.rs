#[cfg(feature = "IoPintOutline")]
use leptos::*;
#[cfg(feature = "IoPintOutline")]
///This icon requires the feature `IoPintOutline` to be enabled.
#[component]
pub fn PintOutline(
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
        "M132.43,162c-6.24-34-4.49-45.55-3.07-68.39L132.27,47a16,16,0,0,1,15.94-15H363.78a16,16,0,0,1,15.94,15l2.91,46.61c1.43,22.86,3.19,34.39-3.06,68.45-5.93,32.29-43.71,133.27-43.71,238.32V472a8,8,0,0,1-8,8H184.12a8,8,0,0,1-8-8V400.37C176.13,307.9,138.66,196.07,132.43,162Z"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "135.21" y1 = "96" x2 =
        "376.79" y2 = "96" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        /> < title > { title } < / title > < / svg >
    }
}
