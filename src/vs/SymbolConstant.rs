#[cfg(feature = "VsSymbolConstant")]
use leptos::*;
#[cfg(feature = "VsSymbolConstant")]
///This icon requires the feature `VsSymbolConstant` to be enabled.
#[component]
pub fn SymbolConstant(
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
        rule = "evenodd" clip - rule = "evenodd" d = "M4 6h8v1H4V6zm8 3H4v1h8V9z" /><
        path xmlns = "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule =
        "evenodd" d = "M1 4l1-1h12l1 1v8l-1 1H2l-1-1V4zm1 0v8h12V4H2z" /> < title > {
        title } < / title > < / svg >
    }
}
