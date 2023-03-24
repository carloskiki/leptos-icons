#[cfg(feature = "TiFilm")]
use leptos::*;
#[cfg(feature = "TiFilm")]
///This icon requires the feature `TiFilm` to be enabled.
#[component]
pub fn Film(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < g xmlns = "http://www.w3.org/2000/svg" >< path
        d =
        "M8 8v7h8v-7h-8zm7 6h-6v-5h6v5zM17 2h-3v2h-4v-2h-3c-1.654 0-3 1.346-3 3v13c0 1.654 1.346 3 3 3h3v-2h4v2h3c1.654 0 3-1.346 3-3v-13c0-1.654-1.346-3-3-3zm1 4c-.553 0-1 .447-1 1s.447 1 1 1v1c-.553 0-1 .447-1 1s.447 1 1 1v1c-.553 0-1 .447-1 1s.447 1 1 1v1c-.553 0-1 .447-1 1s.447 1 1 1v1c0 .551-.448 1-1 1h-1v-2h-8v2h-1c-.552 0-1-.449-1-1v-1c.553 0 1-.447 1-1s-.447-1-1-1v-1c.553 0 1-.447 1-1s-.447-1-1-1v-1c.553 0 1-.447 1-1s-.447-1-1-1v-1c.553 0 1-.447 1-1s-.447-1-1-1v-1c0-.551.448-1 1-1h1v2h8v-2h1c.552 0 1 .449 1 1v1z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
