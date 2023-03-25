#[cfg(feature = "OcLgFileDiff")]
use leptos::*;
#[cfg(feature = "OcLgFileDiff")]
///This icon requires the feature `OcLgFileDiff` to be enabled.
#[component]
pub fn FileDiff(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M12.5 6.75a.75.75 0 0 0-1.5 0V9H8.75a.75.75 0 0 0 0 1.5H11v2.25a.75.75 0 0 0 1.5 0V10.5h2.25a.75.75 0 0 0 0-1.5H12.5V6.75ZM8.75 16a.75.75 0 0 0 0 1.5h6a.75.75 0 0 0 0-1.5h-6Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5 1h9.982a2 2 0 0 1 1.414.586l4.018 4.018A2 2 0 0 1 21 7.018V21a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V3a2 2 0 0 1 2-2Zm-.5 2v18a.5.5 0 0 0 .5.5h14a.5.5 0 0 0 .5-.5V7.018a.5.5 0 0 0-.146-.354l-4.018-4.018a.5.5 0 0 0-.354-.146H5a.5.5 0 0 0-.5.5Z"
        /> < title > { title } < / title > < / svg >
    }
}
