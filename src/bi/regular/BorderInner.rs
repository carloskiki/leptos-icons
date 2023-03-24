#[cfg(feature = "BiRegularBorderInner")]
use leptos::*;
#[cfg(feature = "BiRegularBorderInner")]
///This icon requires the feature `BiRegularBorderInner` to be enabled.
#[component]
pub fn BorderInner(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M19 19h2v2h-2zM7 19h2v2H7zm8 0h2v2h-2zM3 19h2v2H3zm0-4h2v2H3zm0-8h2v2H3zm0-4h2v2H3zm12 0h2v2h-2zM7 3h2v2H7zm12 0h2v2h-2zm0 12h2v2h-2zm0-8h2v2h-2z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5 13h6v8h2v-8h8v-2h-8V3h-2v8H3v2h1.93z" /> < title > { title } < / title > < /
        svg >
    }
}
