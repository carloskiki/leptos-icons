#[cfg(feature = "CgStyle")]
use leptos::*;
#[cfg(feature = "CgStyle")]
///This icon requires the feature `CgStyle` to be enabled.
#[component]
pub fn Style(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M13 21V13H21V21H13ZM15 15H19L19 19H15V15Z" fill = "currentColor" />< path xmlns
        = "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d
        = "M3 11L3 3L11 3V11H3ZM5 5L9 5V9L5 9L5 5Z" fill = "currentColor" />< path xmlns
        = "http://www.w3.org/2000/svg" d = "M18 6V12H16V8L12 8V6L18 6Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 18H6L6 12H8L8 16H12V18Z" fill = "currentColor" /> < title > { title } < /
        title > < / svg >
    }
}
