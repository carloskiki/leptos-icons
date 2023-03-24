#[cfg(feature = "ImFont")]
use leptos::*;
#[cfg(feature = "ImFont")]
///This icon requires the feature `ImFont` to be enabled.
#[component]
pub fn Font(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M12.494 0.253c-1.414 0-2.322-0.253-3.779-0.253-4.708 0-6.903 2.681-6.903 5.404 0 1.604 0.76 2.132 2.259 2.132-0.106-0.232-0.296-0.486-0.296-1.626 0-3.188 1.203-4.117 2.744-4.18 0 0-1.264 12.396-4.934 13.883v0.385h4.947l1.688-8h3.091l0.689-2h-3.358l0.812-3.847c0.929 0.19 1.837 0.38 2.618 0.38 0.971 0 1.858-0.296 2.343-2.533-0.591 0.19-1.224 0.253-1.921 0.253z"
        /> < title > { title } < / title > < / svg >
    }
}
