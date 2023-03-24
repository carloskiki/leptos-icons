#[cfg(feature = "CgTemplate")]
use leptos::*;
#[cfg(feature = "CgTemplate")]
///This icon requires the feature `CgTemplate` to be enabled.
#[component]
pub fn Template(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d = "M3 3V9H21V3H3ZM19 5H5V7H19V5Z" fill
        = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d = "M3 11V21H11V11H3ZM9 13H5V19H9V13Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M21 11H13V13H21V11Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M13 15H21V17H13V15Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M21 19H13V21H21V19Z" fill =
        "currentColor" /> < title > { title } < / title > < / svg >
    }
}
