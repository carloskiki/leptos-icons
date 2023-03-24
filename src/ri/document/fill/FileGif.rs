#[cfg(feature = "RiDocumentFillFileGif")]
use leptos::*;
#[cfg(feature = "RiDocumentFillFileGif")]
///This icon requires the feature `RiDocumentFillFileGif` to be enabled.
#[component]
pub fn FileGif(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0L24 0 24 24 0 24z"
        />< path d =
        "M16 2l5 5v13.993c0 .556-.445 1.007-.993 1.007H3.993C3.445 22 3 21.545 3 21.008V2.992C3 2.444 3.447 2 3.999 2H16zm-3 8h-1v5h1v-5zm-2 0H9c-1.105 0-2 .895-2 2v1c0 1.105.895 2 2 2h1c.552 0 1-.448 1-1v-2H9v1h1v1H9c-.552 0-1-.448-1-1v-1c0-.552.448-1 1-1h2v-1zm6 0h-3v5h1v-2h2v-1h-2v-1h2v-1z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
