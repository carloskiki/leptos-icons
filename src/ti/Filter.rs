#[cfg(feature = "TiFilter")]
use leptos::*;
#[cfg(feature = "TiFilter")]
///This icon requires the feature `TiFilter` to be enabled.
#[component]
pub fn Filter(
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
        stroke_witdh = "0" style = style version = "1" width = "24" height = "24" viewBox
        = "0 0 24 24" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M19 6h-14c-1.1 0-1.4.6-.6 1.4l4.2 4.2c.8.8 1.4 2.3 1.4 3.4v5l4-2v-3.5c0-.8.6-2.1 1.4-2.9l4.2-4.2c.8-.8.5-1.4-.6-1.4z"
        /> < title > { title } < / title > < / svg >
    }
}
