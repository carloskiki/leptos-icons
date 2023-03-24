#[cfg(feature = "TiArchive")]
use leptos::*;
#[cfg(feature = "TiArchive")]
///This icon requires the feature `TiArchive` to be enabled.
#[component]
pub fn Archive(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = { size.clone() } height = { size
        } > < g xmlns = "http://www.w3.org/2000/svg" >< path d =
        "M13 12h-3c-.276 0-.5.224-.5.5s.224.5.5.5h3c.276 0 .5-.224.5-.5s-.224-.5-.5-.5zM20 5h-17c-.553 0-1 .448-1 1s.447 1 1 1h17c.553 0 1-.448 1-1s-.447-1-1-1zM18 8h-13c-.553 0-1 .448-1 1v8c0 1.654 1.346 3 3 3h9c1.654 0 3-1.346 3-3v-8c0-.552-.447-1-1-1zm-2 10h-9c-.552 0-1-.449-1-1v-7h11v7c0 .551-.448 1-1 1z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
