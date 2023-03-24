#[cfg(feature = "IoFolderSharp")]
use leptos::*;
#[cfg(feature = "IoFolderSharp")]
///This icon requires the feature `IoFolderSharp` to be enabled.
#[component]
pub fn FolderSharp(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M16,420a28,28,0,0,0,28,28H468a28,28,0,0,0,28-28V208H16Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M496,124a28,28,0,0,0-28-28H212.84l-48-32H44A28,28,0,0,0,16,92v84H496Z" /> <
        title > { title } < / title > < / svg >
    }
}
