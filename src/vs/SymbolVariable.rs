#[cfg(feature = "VsSymbolVariable")]
use leptos::*;
#[cfg(feature = "VsSymbolVariable")]
///This icon requires the feature `VsSymbolVariable` to be enabled.
#[component]
pub fn SymbolVariable(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M2 5h2V4H1.5l-.5.5v8l.5.5H4v-1H2V5zm12.5-1H12v1h2v7h-2v1h2.5l.5-.5v-8l-.5-.5zm-2.74 2.57L12 7v2.51l-.3.45-4.5 2h-.46l-2.5-1.5-.24-.43v-2.5l.3-.46 4.5-2h.46l2.5 1.5zM5 9.71l1.5.9V9.28L5 8.38v1.33zm.58-2.15l1.45.87 3.39-1.5-1.45-.87-3.39 1.5zm1.95 3.17l3.5-1.56v-1.4l-3.5 1.55v1.41z"
        /> < title > { title } < / title > < / svg >
    }
}
