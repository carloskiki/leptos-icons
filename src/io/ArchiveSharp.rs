#[cfg(feature = "IoArchiveSharp")]
use leptos::*;
#[cfg(feature = "IoArchiveSharp")]
///This icon requires the feature `IoArchiveSharp` to be enabled.
#[component]
pub fn ArchiveSharp(
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
        "http://www.w3.org/2000/svg" > < rect xmlns = "http://www.w3.org/2000/svg" x =
        "32" y = "48" width = "448" height = "80" rx = "12" ry = "12" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M64,160V440a24,24,0,0,0,24,24H424a24,24,0,0,0,24-24V160ZM256,390.63,169.32,304,192,281.32,240,329.37V208h32V329.37l48.07-48.07,22.61,22.64Z"
        /> < title > { title } < / title > < / svg >
    }
}
