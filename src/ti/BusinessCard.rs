#[cfg(feature = "TiBusinessCard")]
use leptos::*;
#[cfg(feature = "TiBusinessCard")]
///This icon requires the feature `TiBusinessCard` to be enabled.
#[component]
pub fn BusinessCard(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < g xmlns = "http://www.w3.org/2000/svg" >< path
        d =
        "M20 20h-16c-1.654 0-3-1.346-3-3v-10c0-1.654 1.346-3 3-3h16c1.654 0 3 1.346 3 3v10c0 1.654-1.346 3-3 3zm-16-14c-.551 0-1 .449-1 1v10c0 .551.449 1 1 1h16c.551 0 1-.449 1-1v-10c0-.551-.449-1-1-1h-16zM10 15h-4c-.553 0-1-.448-1-1s.447-1 1-1h4c.553 0 1 .448 1 1s-.447 1-1 1zM10 11h-4c-.553 0-1-.448-1-1s.447-1 1-1h4c.553 0 1 .448 1 1s-.447 1-1 1z"
        />< circle cx = "16" cy = "10.5" r = "2" />< path d =
        "M16 13.356c-1.562 0-2.5.715-2.5 1.429 0 .357.938.715 2.5.715 1.466 0 2.5-.357 2.5-.715 0-.714-.98-1.429-2.5-1.429z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
