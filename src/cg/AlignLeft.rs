#[cfg(feature = "CgAlignLeft")]
use leptos::*;
#[cfg(feature = "CgAlignLeft")]
///This icon requires the feature `CgAlignLeft` to be enabled.
#[component]
pub fn AlignLeft(
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
        "M8 13H14V17H8V13Z" fill = "currentColor" fill - opacity = "0.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M6 6H4V18H6V6Z" fill = "currentColor" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M20 7H8V11H20V7Z" fill = "currentColor"
        /> < title > { title } < / title > < / svg >
    }
}
