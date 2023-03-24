#[cfg(feature = "CgLayoutGrid")]
use leptos::*;
#[cfg(feature = "CgLayoutGrid")]
///This icon requires the feature `CgLayoutGrid` to be enabled.
#[component]
pub fn LayoutGrid(
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
        "M11 7H7V11H11V7Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M11 13H7V17H11V13Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M13 13H17V17H13V13Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M17 7H13V11H17V7Z" fill = "currentColor" /> < title > { title } < / title > < /
        svg >
    }
}
