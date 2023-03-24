#[cfg(feature = "TiKeyboard")]
use leptos::*;
#[cfg(feature = "TiKeyboard")]
///This icon requires the feature `TiKeyboard` to be enabled.
#[component]
pub fn Keyboard(
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
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 13h7v2h-7zM5 13h2v2h-2zM5 9h2v1h-2zM8 12v-1h-3v1h2zM8 9h1v1h-1zM9 11h1v1h-1zM10 9h1v1h-1zM11 11h1v1h-1zM12 9h1v1h-1zM13 11h1v1h-1zM14 9h1v1h-1zM15 11h1v1h-1zM16 9h1v1h-1zM17 12h2v-3h-1v2h-1zM18 13h-1v1h-1v1h3v-1h-1zM20 6h-16c-1.1 0-2 .9-2 2v8c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2v-8c0-1.1-.9-2-2-2zm0 10h-16v-8h16v8z"
        /> < title > { title } < / title > < / svg >
    }
}
