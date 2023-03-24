#[cfg(feature = "ImMeter")]
use leptos::*;
#[cfg(feature = "ImMeter")]
///This icon requires the feature `ImMeter` to be enabled.
#[component]
pub fn Meter(
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
        "M8 1c4.418 0 8 3.582 8 8 0 3.012-1.665 5.635-4.125 7h-7.75c-2.46-1.365-4.125-3.988-4.125-7 0-4.418 3.582-8 8-8zM12.53 13.53c1.21-1.21 1.876-2.819 1.876-4.53h-1.406v-1h1.329c-0.11-0.703-0.334-1.377-0.665-2h-1.664v-1h1.004c-0.147-0.184-0.306-0.361-0.475-0.53-0.722-0.722-1.587-1.251-2.53-1.559v1.089h-1v-1.329c-0.328-0.051-0.662-0.078-1-0.078s-0.672 0.026-1 0.078v1.329h-1v-1.089c-0.943 0.309-1.808 0.837-2.53 1.559-0.169 0.169-0.327 0.346-0.475 0.53h1.004v1h-1.664c-0.331 0.623-0.555 1.297-0.665 2h1.329v1h-1.406c0 1.711 0.666 3.32 1.876 4.53 0.167 0.167 0.343 0.324 0.524 0.47h3.006l0.571-8h0.857l0.571 8h3.006c0.182-0.146 0.357-0.303 0.524-0.47z"
        /> < title > { title } < / title > < / svg >
    }
}
