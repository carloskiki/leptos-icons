#[cfg(feature = "FaSolidLaptopFile")]
use leptos::*;
#[cfg(feature = "FaSolidLaptopFile")]
///This icon requires the feature `FaSolidLaptopFile` to be enabled.
#[component]
pub fn LaptopFile(
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
        stroke_witdh = "0" style = style viewBox = "0 0 640 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M192 0H48C21.5 0 0 21.5 0 48V368c0 26.5 21.5 48 48 48H162.7c6.6-18.6 24.4-32 45.3-32V272c0-44.2 35.8-80 80-80h32V128H224c-17.7 0-32-14.3-32-32V0zm96 224c-26.5 0-48 21.5-48 48v16 96 32H208c-8.8 0-16 7.2-16 16v16c0 35.3 28.7 64 64 64H576c35.3 0 64-28.7 64-64V432c0-8.8-7.2-16-16-16H592V288c0-35.3-28.7-64-64-64H320 304 288zm32 64H528V416H304V288h16zM224 0V96h96L224 0z"
        /> < title > { title } < / title > < / svg >
    }
}
