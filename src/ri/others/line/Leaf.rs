#[cfg(feature = "RiOthersLineLeaf")]
use leptos::*;
#[cfg(feature = "RiOthersLineLeaf")]
///This icon requires the feature `RiOthersLineLeaf` to be enabled.
#[component]
pub fn Leaf(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0H24V24H0z" />< path d
        =
        "M21 3v2c0 9.627-5.373 14-12 14H5.243C5.08 19.912 5 20.907 5 22H3c0-1.363.116-2.6.346-3.732C3.116 16.974 3 15.218 3 13 3 7.477 7.477 3 13 3c2 0 4 1 8 0zm-8 2c-4.418 0-8 3.582-8 8 0 .362.003.711.01 1.046 1.254-1.978 3.091-3.541 5.494-4.914l.992 1.736C8.641 12.5 6.747 14.354 5.776 17H9c6.015 0 9.871-3.973 9.997-11.612-1.372.133-2.647.048-4.22-.188C13.627 5.027 13.401 5 13 5z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
