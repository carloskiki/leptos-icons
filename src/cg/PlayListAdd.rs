#[cfg(feature = "CgPlayListAdd")]
use leptos::*;
#[cfg(feature = "CgPlayListAdd")]
///This icon requires the feature `CgPlayListAdd` to be enabled.
#[component]
pub fn PlayListAdd(
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
        "M2 5H14V7H2V5Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M2 9H14V11H2V9Z" fill = "currentColor" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M10 13H2V15H10V13Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M16 9H18V13H22V15H18V19H16V15H12V13H16V9Z" fill = "currentColor" /> < title > {
        title } < / title > < / svg >
    }
}
