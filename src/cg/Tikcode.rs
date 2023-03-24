#[cfg(feature = "CgTikcode")]
use leptos::*;
#[cfg(feature = "CgTikcode")]
///This icon requires the feature `CgTikcode` to be enabled.
#[component]
pub fn Tikcode(
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
        rule = "evenodd" clip - rule = "evenodd" d = "M9 5H5V9H9V5ZM3 3V11H11V3H3Z" fill
        = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d = "M19 5H15V9H19V5ZM13 3V11H21V3H13Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d = "M9 15H5V19H9V15ZM3 13V21H11V13H3Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M13 13H15V21H13V13Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 13H18V21H16V13Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M19 13H21V21H19V13Z" fill =
        "currentColor" /> < title > { title } < / title > < / svg >
    }
}
