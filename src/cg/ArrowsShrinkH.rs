#[cfg(feature = "CgArrowsShrinkH")]
use leptos::*;
#[cfg(feature = "CgArrowsShrinkH")]
///This icon requires the feature `CgArrowsShrinkH` to be enabled.
#[component]
pub fn ArrowsShrinkH(
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
        "M1 7H3V17H1V7Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M8.44769 7.75732L9.86191 9.17154L8.03344 11H15.9664L14.138 9.17154L15.5522 7.75732L19.7948 12L15.5522 16.2426L14.138 14.8284L15.9664 13H8.03354L9.86194 14.8284L8.44773 16.2426L4.20505 12L8.44769 7.75732Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M23 7H21V17H23V7Z" fill = "currentColor" /> < title > { title } < / title > < /
        svg >
    }
}
