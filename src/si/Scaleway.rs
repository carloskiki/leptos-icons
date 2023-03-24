#[cfg(feature = "SiScaleway")]
use leptos::*;
#[cfg(feature = "SiScaleway")]
///This icon requires the feature `SiScaleway` to be enabled.
#[component]
pub fn Scaleway(
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
        "M16.605 11.11v5.72a1.77 1.77 0 01-1.54 1.69h-4a1.43 1.43 0 01-1.31-1.22 1.09 1.09 0 010-.18 1.37 1.37 0 011.37-1.36h1.74a1 1 0 001-1v-3.62a1.4 1.4 0 011.18-1.39h.17a1.37 1.37 0 011.39 1.36zm-6.46 1.74V9.26a1 1 0 011-1h1.85a1.37 1.37 0 001.37-1.37 1 1 0 000-.17 1.45 1.45 0 00-1.41-1.2h-3.96a1.81 1.81 0 00-1.58 1.66v5.7a1.37 1.37 0 001.37 1.37h.21a1.4 1.4 0 001.15-1.4zm12-4.29V20a4.53 4.53 0 01-4.15 4h-7.58a8.57 8.57 0 01-8.56-8.57V4.54A4.54 4.54 0 016.395 0h7.18a8.56 8.56 0 018.56 8.56zm-2.74 0a5.83 5.83 0 00-5.82-5.82h-7.19a1.79 1.79 0 00-1.8 1.8v10.89a5.83 5.83 0 005.82 5.8h7.44a1.79 1.79 0 001.54-1.48z"
        /> < title > { title } < / title > < / svg >
    }
}
