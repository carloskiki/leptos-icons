#[cfg(feature = "CgToggleOff")]
use leptos::*;
#[cfg(feature = "CgToggleOff")]
///This icon requires the feature `CgToggleOff` to be enabled.
#[component]
pub fn ToggleOff(
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
        "M17 15C18.6569 15 20 13.6569 20 12C20 10.3431 18.6569 9 17 9C15.3431 9 14 10.3431 14 12C14 13.6569 15.3431 15 17 15Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M0 12C0 8.13401 3.13401 5 7 5H17C20.866 5 24 8.13401 24 12C24 15.866 20.866 19 17 19H7C3.13401 19 0 15.866 0 12ZM7 7H17C19.7614 7 22 9.23858 22 12C22 14.7614 19.7614 17 17 17H7C4.23858 17 2 14.7614 2 12C2 9.23858 4.23858 7 7 7Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
