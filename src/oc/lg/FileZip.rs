#[cfg(feature = "OcLgFileZip")]
use leptos::*;
#[cfg(feature = "OcLgFileZip")]
///This icon requires the feature `OcLgFileZip` to be enabled.
#[component]
pub fn FileZip(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M5 2.5a.5.5 0 0 0-.5.5v18a.5.5 0 0 0 .5.5h1.75a.75.75 0 0 1 0 1.5H5a2 2 0 0 1-2-2V3a2 2 0 0 1 2-2h9.982a2 2 0 0 1 1.414.586l4.018 4.018A2 2 0 0 1 21 7.018V21a2 2 0 0 1-2 2h-2.75a.75.75 0 0 1 0-1.5H19a.5.5 0 0 0 .5-.5V7.018a.5.5 0 0 0-.146-.354l-4.018-4.018a.5.5 0 0 0-.354-.146H5Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M11.5 15.75a.75.75 0 0 1 .75-.75h1a.75.75 0 0 1 0 1.5h-1a.75.75 0 0 1-.75-.75Zm.75-3.75a.75.75 0 0 0 0 1.5h1a.75.75 0 0 0 0-1.5h-1Zm-.75-2.25a.75.75 0 0 1 .75-.75h1a.75.75 0 0 1 0 1.5h-1a.75.75 0 0 1-.75-.75ZM12.25 6a.75.75 0 0 0 0 1.5h1a.75.75 0 0 0 0-1.5h-1Zm-.75-2.25a.75.75 0 0 1 .75-.75h1a.75.75 0 0 1 0 1.5h-1a.75.75 0 0 1-.75-.75ZM9.75 13.5a.75.75 0 0 0 0 1.5h1a.75.75 0 0 0 0-1.5h-1ZM9 11.25a.75.75 0 0 1 .75-.75h1a.75.75 0 0 1 0 1.5h-1a.75.75 0 0 1-.75-.75Zm.75-3.75a.75.75 0 0 0 0 1.5h1a.75.75 0 0 0 0-1.5h-1ZM9 5.25a.75.75 0 0 1 .75-.75h1a.75.75 0 0 1 0 1.5h-1A.75.75 0 0 1 9 5.25ZM11 17h1a2 2 0 0 1 2 2v4.25a.75.75 0 0 1-.75.75h-3.5a.75.75 0 0 1-.75-.75V19a2 2 0 0 1 2-2Zm-.5 2v3.5h2V19a.5.5 0 0 0-.5-.5h-1a.5.5 0 0 0-.5.5Z"
        /> < title > { title } < / title > < / svg >
    }
}
