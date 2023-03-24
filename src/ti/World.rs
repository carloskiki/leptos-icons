#[cfg(feature = "TiWorld")]
use leptos::*;
#[cfg(feature = "TiWorld")]
///This icon requires the feature `TiWorld` to be enabled.
#[component]
pub fn World(
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
        "M12 2c-4.971 0-9 4.029-9 9s4.029 9 9 9 9-4.029 9-9-4.029-9-9-9zm2 2c0 1-.5 2-1.5 2s-1.5 1-1.5 2v3s1 0 1-3c0-.553.447-1 1-1s1 .447 1 1v3c-.552 0-1 .448-1 1s.448 1 1 1c.553 0 1-.448 1-1h1v-2l1 1-1 1c0 3 0 3-2 4 0-1-1-1-3-1v-2l-2-2v-2c-1 0-1 1-1 1l-.561-.561-2.39-2.39c.11-.192.225-.382.35-.564l.523-.678c1.468-1.716 3.644-2.807 6.078-2.807.691 0 1.359.098 2 .262v.738z"
        /> < title > { title } < / title > < / svg >
    }
}
