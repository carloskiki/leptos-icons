#[cfg(feature = "CgDisplayGrid")]
use leptos::*;
#[cfg(feature = "CgDisplayGrid")]
///This icon requires the feature `CgDisplayGrid` to be enabled.
#[component]
pub fn DisplayGrid(
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
        "M7 7V11H11V7H7Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M13 7H17V11H13V7Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M13 13V17H17V13H13Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7 13H11V17H7V13Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M3 3H21V21H3V3ZM5 5V19H19V5H5Z" fill = "currentColor" /> < title > { title } < /
        title > < / svg >
    }
}
