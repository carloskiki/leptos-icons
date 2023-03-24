#[cfg(feature = "CgFormatIndentDecrease")]
use leptos::*;
#[cfg(feature = "CgFormatIndentDecrease")]
///This icon requires the feature `CgFormatIndentDecrease` to be enabled.
#[component]
pub fn FormatIndentDecrease(
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
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M20 7H4V5H20V7Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 11H12V9H20V11Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M12 15H20V13H12V15Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9 9L4 12L9 15V9Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 17V19H20V17H4Z" fill = "currentColor" /> <
        title > { title } < / title > < / svg >
    }
}
