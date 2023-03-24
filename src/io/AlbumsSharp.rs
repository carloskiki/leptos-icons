#[cfg(feature = "IoAlbumsSharp")]
use leptos::*;
#[cfg(feature = "IoAlbumsSharp")]
///This icon requires the feature `IoAlbumsSharp` to be enabled.
#[component]
pub fn AlbumsSharp(
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
        "0 0 512 512" width = { size.clone() } height = { size } > < rect xmlns =
        "http://www.w3.org/2000/svg" x = "128" y = "64" width = "256" height = "32" /><
        rect xmlns = "http://www.w3.org/2000/svg" x = "96" y = "112" width = "320" height
        = "32" />< path xmlns = "http://www.w3.org/2000/svg" d = "M464,448H48V160H464Z"
        /> < title > { title } < / title > < / svg >
    }
}
