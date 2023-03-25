#[cfg(feature = "FaSolidTachographDigital")]
use leptos::*;
#[cfg(feature = "FaSolidTachographDigital")]
///This icon requires the feature `FaSolidTachographDigital` to be enabled.
#[component]
pub fn TachographDigital(
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
        "M64 64C28.7 64 0 92.7 0 128V384c0 35.3 28.7 64 64 64H576c35.3 0 64-28.7 64-64V128c0-35.3-28.7-64-64-64H64zm32 64H320c17.7 0 32 14.3 32 32v64c0 17.7-14.3 32-32 32H96c-17.7 0-32-14.3-32-32V160c0-17.7 14.3-32 32-32zM64 368c0-8.8 7.2-16 16-16H336c8.8 0 16 7.2 16 16s-7.2 16-16 16H80c-8.8 0-16-7.2-16-16zm320 0c0-8.8 7.2-16 16-16H560c8.8 0 16 7.2 16 16s-7.2 16-16 16H400c-8.8 0-16-7.2-16-16zM80 288a16 16 0 1 1 0 32 16 16 0 1 1 0-32zm48 16a16 16 0 1 1 32 0 16 16 0 1 1 -32 0zm80-16a16 16 0 1 1 0 32 16 16 0 1 1 0-32zm48 16a16 16 0 1 1 32 0 16 16 0 1 1 -32 0zm80-16a16 16 0 1 1 0 32 16 16 0 1 1 0-32z"
        /> < title > { title } < / title > < / svg >
    }
}
