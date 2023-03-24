#[cfg(feature = "ImPaste")]
use leptos::*;
#[cfg(feature = "ImPaste")]
///This icon requires the feature `ImPaste` to be enabled.
#[component]
pub fn Paste(
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
        "M11 2h-2v-1c0-0.55-0.45-1-1-1h-2c-0.55 0-1 0.45-1 1v1h-2v2h8v-2zM8 2h-2v-0.998c0.001-0.001 0.001-0.001 0.002-0.002h1.996c0.001 0.001 0.001 0.001 0.002 0.002v0.998zM13 5v-2.5c0-0.275-0.225-0.5-0.5-0.5h-1v1h0.5v2h-3l-3 3v4h-4v-9h0.5v-1h-1c-0.275 0-0.5 0.225-0.5 0.5v10c0 0.275 0.225 0.5 0.5 0.5h4.5v3h10v-11h-3zM9 6.414v1.586h-1.586l1.586-1.586zM15 15h-8v-6h3v-3h5v9z"
        /> < title > { title } < / title > < / svg >
    }
}
