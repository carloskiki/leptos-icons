#[cfg(feature = "RiDocumentFillFolderShield")]
use leptos::*;
#[cfg(feature = "RiDocumentFillFolderShield")]
///This icon requires the feature `RiDocumentFillFolderShield` to be enabled.
#[component]
pub fn FolderShield(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path
        fill - rule = "nonzero" d =
        "M12.414 5H21a1 1 0 0 1 1 1v14a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h7.414l2 2zM8 9v4.904c0 .892.446 1.724 1.187 2.219L12 17.998l2.813-1.875A2.667 2.667 0 0 0 16 13.904V9H8zm2 4.904V11h4v2.904a.667.667 0 0 1-.297.555L12 15.594l-1.703-1.135a.667.667 0 0 1-.297-.555z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
