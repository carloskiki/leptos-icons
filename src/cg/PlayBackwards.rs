#[cfg(feature = "CgPlayBackwards")]
use leptos::*;
#[cfg(feature = "CgPlayBackwards")]
///This icon requires the feature `CgPlayBackwards` to be enabled.
#[component]
pub fn PlayBackwards(
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
        "M2 7H5V17H2V7Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M6 12L13.0023 7.00003V17L6 12Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M21.0023 7.00003L14 12L21.0023 17V7.00003Z" fill = "currentColor" /> < title > {
        title } < / title > < / svg >
    }
}
