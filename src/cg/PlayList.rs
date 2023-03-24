#[cfg(feature = "CgPlayList")]
use leptos::*;
#[cfg(feature = "CgPlayList")]
///This icon requires the feature `CgPlayList` to be enabled.
#[component]
pub fn PlayList(
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
        "M16 5H4V7H16V5Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 9H4V11H16V9Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M4 13H12V15H4V13Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M20 16L14 13V19L20 16Z" fill = "currentColor" /> < title > { title } < / title >
        < / svg >
    }
}
