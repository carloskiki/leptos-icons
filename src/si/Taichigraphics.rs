#[cfg(feature = "SiTaichigraphics")]
use leptos::*;
#[cfg(feature = "SiTaichigraphics")]
///This icon requires the feature `SiTaichigraphics` to be enabled.
#[component]
pub fn Taichigraphics(
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
        "M19.343 20.672a1.94 1.94 0 0 0 1.94-1.94 1.94 1.94 0 1 0-3.88 0 1.94 1.94 0 0 0 1.94 1.94zM9.058 12.796a6.858 6.858 0 1 0-2.324-9.67c-.062.099-.125.198-.185.3-.06.103-.11.205-.167.31a6.858 6.858 0 0 0 2.676 9.06zm0-.003h-4.23l-2.113 3.663 2.114 3.667h4.229l2.116-3.667zm0 7.33L6.82 23.999h4.48Z"
        /> < title > { title } < / title > < / svg >
    }
}
