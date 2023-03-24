#[cfg(feature = "VsSymbolEvent")]
use leptos::*;
#[cfg(feature = "VsSymbolEvent")]
///This icon requires the feature `VsSymbolEvent` to be enabled.
#[component]
pub fn SymbolEvent(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M7.414 1.56L8.312 1h3.294l.818 1.575L10.236 6h1.781l.72 1.695L5.618 15l-1.602-1.163L6.119 10H4.898L4 8.56l3.414-7zM7.78 9L4.9 14.305 12.018 7H8.312l3.294-5H8.312L4.898 9H7.78z"
        /> < title > { title } < / title > < / svg >
    }
}
