#[cfg(feature = "FaSolidMound")]
use leptos::*;
#[cfg(feature = "FaSolidMound")]
///This icon requires the feature `FaSolidMound` to be enabled.
#[component]
pub fn Mound(
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
        stroke_witdh = "0" style = style viewBox = "0 0 576 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M113.1 179.2C142.8 127.7 197.6 96 257 96s114.2 31.7 143.9 83.2L509.4 368c12.3 21.3-3.1 48-27.7 48H32.3c-24.6 0-40-26.6-27.7-48L113.1 179.2z"
        /> < title > { title } < / title > < / svg >
    }
}
