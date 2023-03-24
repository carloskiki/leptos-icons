#[cfg(feature = "SiMoleculer")]
use leptos::*;
#[cfg(feature = "SiMoleculer")]
///This icon requires the feature `SiMoleculer` to be enabled.
#[component]
pub fn Moleculer(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M15.442.718a2.58 2.58 0 0 0-2.579 2.579 2.58 2.58 0 0 0 1.368 2.275L12.809 8.27a3.505 3.505 0 0 0-1.077-.172 3.505 3.505 0 0 0-3.505 3.505 3.505 3.505 0 0 0 .085.745l-2.83 1.036a2.97 2.97 0 0 0-2.513-1.39A2.97 2.97 0 0 0 0 14.962a2.97 2.97 0 0 0 2.97 2.97 2.97 2.97 0 0 0 2.969-2.97 2.97 2.97 0 0 0-.072-.634l2.716-1.193a3.505 3.505 0 0 0 3.15 1.972 3.505 3.505 0 0 0 2.129-.724l2.276 2.167a4.305 4.305 0 0 0-.749 2.426 4.305 4.305 0 0 0 4.306 4.305A4.305 4.305 0 0 0 24 18.977a4.305 4.305 0 0 0-4.305-4.305 4.305 4.305 0 0 0-2.718.969l-2.424-1.964a3.505 3.505 0 0 0 .684-2.074 3.505 3.505 0 0 0-1.521-2.89l1.204-2.891a2.58 2.58 0 0 0 .522.054 2.58 2.58 0 0 0 2.58-2.58 2.58 2.58 0 0 0-2.58-2.578Z"
        /> < title > { title } < / title > < / svg >
    }
}
