#[cfg(feature = "SiZerply")]
use leptos::*;
#[cfg(feature = "SiZerply")]
///This icon requires the feature `SiZerply` to be enabled.
#[component]
pub fn Zerply(
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
        "M20.779 18.746c-.747.714-1.562 1.017-2.543 1.017-1.32 0-3.322-.61-4.846-1.66-1.629-1.119-3.765-2.237-5.562-2.271 1.323-1.798 3.39-3.628 5.322-5.798.713-.78 4.983-5.7 5.73-6.586.54-.645.813-1.424.813-2.205 0-.3-.033-.585-.101-.855-2.035.405-3.561.601-6.001.601-2.677.015-4.607-.314-5.73-.989-.78 1.018-1.56 2.373-1.56 3.12 0 .948.918 1.728 3.189 1.728.746 0 1.965-.034 3.66-.169-3.492 4.5-6.949 8.16-9.016 10.47-.713.781-1.121 1.83-1.121 2.881 0 .405.075.81.18 1.185.645-.104 1.291-.179 1.965-.179 1.395 0 2.79.299 4.081.839C11.805 21.014 14.205 24 16.921 24c2.204 0 4.065-1.741 4.065-4.036 0-.404-.061-.825-.195-1.229l-.012.011z"
        /> < title > { title } < / title > < / svg >
    }
}
