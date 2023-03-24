#[cfg(feature = "VsTerminalUbuntu")]
use leptos::*;
#[cfg(feature = "VsTerminalUbuntu")]
///This icon requires the feature `VsTerminalUbuntu` to be enabled.
#[component]
pub fn TerminalUbuntu(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.26 8A1.37 1.37 0 1 1 .52 8a1.37 1.37 0 0 1 2.74 0zm7.79 6.66a1.37 1.37 0 1 0 2.374-1.37 1.37 1.37 0 0 0-2.374 1.37zm2.37-11.95a1.37 1.37 0 1 0-2.37-1.373 1.37 1.37 0 0 0 2.37 1.373zM8.79 4.1a3.9 3.9 0 0 1 3.89 3.55h2a5.93 5.93 0 0 0-1.73-3.8 1.91 1.91 0 0 1-1.66-.12 2.001 2.001 0 0 1-.94-1.38 6 6 0 0 0-1.54-.2 5.83 5.83 0 0 0-2.61.61l1 1.73a3.94 3.94 0 0 1 1.59-.39zM4.88 8a3.93 3.93 0 0 1 1.66-3.2l-1-1.7A5.93 5.93 0 0 0 3.1 6.5a1.92 1.92 0 0 1 0 3 5.93 5.93 0 0 0 2.42 3.4l1-1.7A3.93 3.93 0 0 1 4.88 8zm3.91 3.91a4 4 0 0 1-1.65-.37l-1 1.73c.81.403 1.704.612 2.61.61.52 0 1.038-.067 1.54-.2a2 2 0 0 1 .94-1.38 1.911 1.911 0 0 1 1.66-.12 5.93 5.93 0 0 0 1.73-3.8h-2a3.91 3.91 0 0 1-3.83 3.53z"
        /> < title > { title } < / title > < / svg >
    }
}
