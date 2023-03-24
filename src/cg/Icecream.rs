#[cfg(feature = "CgIcecream")]
use leptos::*;
#[cfg(feature = "CgIcecream")]
///This icon requires the feature `CgIcecream` to be enabled.
#[component]
pub fn Icecream(
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
        "M15 17H19V8C19 4.13401 15.866 1 12 1C8.13401 1 5 4.13401 5 8V17H9V20C9 21.6569 10.3431 23 12 23C13.6569 23 15 21.6569 15 20V17ZM17 15V8C17 5.23858 14.7614 3 12 3C9.23858 3 7 5.23858 7 8V15H11V20C11 20.5523 11.4477 21 12 21C12.5523 21 13 20.5523 13 20V15H17Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
