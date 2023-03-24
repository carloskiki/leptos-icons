#[cfg(feature = "CgBorderStyleDotted")]
use leptos::*;
#[cfg(feature = "CgBorderStyleDotted")]
///This icon requires the feature `CgBorderStyleDotted` to be enabled.
#[component]
pub fn BorderStyleDotted(
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
        "M3 11H1V13H3V11Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7 11H5V13H7V11Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M9 11H11V13H9V11Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15 11H13V13H15V11Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 11H19V13H17V11Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M23 11H21V13H23V11Z" fill =
        "currentColor" /> < title > { title } < / title > < / svg >
    }
}
