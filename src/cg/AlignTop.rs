#[cfg(feature = "CgAlignTop")]
use leptos::*;
#[cfg(feature = "CgAlignTop")]
///This icon requires the feature `CgAlignTop` to be enabled.
#[component]
pub fn AlignTop(
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
        rule = "evenodd" clip - rule = "evenodd" d =
        "M13.0352 7.98797L13.0374 13.988L17.0374 13.9865L17.0352 7.98653L13.0352 7.98797Z"
        fill = "currentColor" fill - opacity = "0.5" />< path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M17.9996 4.01189L5.99963 4.01837L6.00038 6.01837L18.0004 6.01189L17.9996 4.01189Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M11.0374 19.9864L11.0321 7.98637L7.03207 7.98813L7.03736 19.9881L11.0374 19.9864Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
