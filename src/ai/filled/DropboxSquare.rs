#[cfg(feature = "AiFilledDropboxSquare")]
use leptos::*;
#[cfg(feature = "AiFilledDropboxSquare")]
///This icon requires the feature `AiFilledDropboxSquare` to be enabled.
#[component]
pub fn DropboxSquare(
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
        stroke_witdh = "0" style = style class = "icon" viewBox = "0 0 1024 1024" width =
        size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M880 112H144c-17.7 0-32 14.3-32 32v736c0 17.7 14.3 32 32 32h736c17.7 0 32-14.3 32-32V144c0-17.7-14.3-32-32-32zM663.2 659.5L512.6 750l-151-90.5v-33.1l45.4 29.4 105.6-87.7 105.6 87.7 45.1-29.4v33.1zm-45.6-22.4l-105.3-87.7L407 637.1l-151-99.2 104.5-82.4L256 371.2 407 274l105.3 87.7L617.6 274 768 372.1l-104.2 83.5L768 539l-150.4 98.1zM512.3 361.7l-151.8 93.8 151.8 93.9 151.5-93.9zm151.5 93.8z"
        /> < title > { title } < / title > < / svg >
    }
}
