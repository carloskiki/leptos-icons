#[cfg(feature = "ImBinoculars")]
use leptos::*;
#[cfg(feature = "ImBinoculars")]
///This icon requires the feature `ImBinoculars` to be enabled.
#[component]
pub fn Binoculars(
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
        "M1 0h6v1h-6zM9 0h6v1h-6zM14.875 5h-0.875v-4h-4v4h-4v-4h-4v4h-0.875c-0.619 0-1.125 0.506-1.125 1.125v8.75c0 0.619 0.506 1.125 1.125 1.125h4.75c0.619 0 1.125-0.506 1.125-1.125v-5.875h2v5.875c0 0.619 0.506 1.125 1.125 1.125h4.75c0.619 0 1.125-0.506 1.125-1.125v-8.75c0-0.619-0.506-1.125-1.125-1.125zM5.438 15h-3.875c-0.309 0-0.563-0.225-0.563-0.5s0.253-0.5 0.563-0.5h3.875c0.309 0 0.563 0.225 0.563 0.5s-0.253 0.5-0.563 0.5zM8.5 8h-1c-0.275 0-0.5-0.225-0.5-0.5s0.225-0.5 0.5-0.5h1c0.275 0 0.5 0.225 0.5 0.5s-0.225 0.5-0.5 0.5zM14.438 15h-3.875c-0.309 0-0.563-0.225-0.563-0.5s0.253-0.5 0.563-0.5h3.875c0.309 0 0.563 0.225 0.563 0.5s-0.253 0.5-0.563 0.5z"
        /> < title > { title } < / title > < / svg >
    }
}
