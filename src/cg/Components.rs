#[cfg(feature = "CgComponents")]
use leptos::*;
#[cfg(feature = "CgComponents")]
///This icon requires the feature `CgComponents` to be enabled.
#[component]
pub fn Components(
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
        "M7.7572 6.3431L11.9998 2.10046L16.2425 6.3431L11.9998 10.5857L7.7572 6.3431ZM10.5856 6.3431L11.9998 4.92889L13.4141 6.3431L11.9998 7.75732L10.5856 6.3431Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M2.10046 11.9999L6.3431 7.75726L10.5857 11.9999L6.3431 16.2425L2.10046 11.9999ZM4.92889 11.9999L6.3431 10.5857L7.75732 11.9999L6.3431 13.4141L4.92889 11.9999Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M13.4142 11.9999L17.6568 16.2425L21.8995 11.9999L17.6568 7.75726L13.4142 11.9999ZM17.6568 10.5857L16.2426 11.9999L17.6568 13.4141L19.071 11.9999L17.6568 10.5857Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M7.7572 17.6569L11.9998 13.4142L16.2425 17.6569L11.9998 21.8995L7.7572 17.6569ZM10.5856 17.6569L11.9998 16.2427L13.4141 17.6569L11.9998 19.0711L10.5856 17.6569Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
