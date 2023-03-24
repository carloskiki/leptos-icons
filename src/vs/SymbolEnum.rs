#[cfg(feature = "VsSymbolEnum")]
use leptos::*;
#[cfg(feature = "VsSymbolEnum")]
///This icon requires the feature `VsSymbolEnum` to be enabled.
#[component]
pub fn SymbolEnum(
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
        "M14 2H8L7 3v3h1V3h6v5h-4v1h4l1-1V3l-1-1zM9 6h4v1H9.41L9 6.59V6zM7 7H2L1 8v5l1 1h6l1-1V8L8 7H7zm1 6H2V8h6v5zM3 9h4v1H3V9zm0 2h4v1H3v-1zm6-7h4v1H9V4z"
        /> < title > { title } < / title > < / svg >
    }
}
