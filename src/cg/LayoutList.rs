#[cfg(feature = "CgLayoutList")]
use leptos::*;
#[cfg(feature = "CgLayoutList")]
///This icon requires the feature `CgLayoutList` to be enabled.
#[component]
pub fn LayoutList(
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
        "M9 7H7V9H9V7Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7 13V11H9V13H7Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M7 15V17H9V15H7Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M11 15V17H17V15H11Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 13V11H11V13H17Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M17 7V9H11V7H17Z" fill =
        "currentColor" /> < title > { title } < / title > < / svg >
    }
}
