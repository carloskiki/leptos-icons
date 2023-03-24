#[cfg(feature = "CgScreen")]
use leptos::*;
#[cfg(feature = "CgScreen")]
///This icon requires the feature `CgScreen` to be enabled.
#[component]
pub fn Screen(
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
        "M11 17H4C2.34315 17 1 15.6569 1 14V6C1 4.34315 2.34315 3 4 3H20C21.6569 3 23 4.34315 23 6V14C23 15.6569 21.6569 17 20 17H13V19H16C16.5523 19 17 19.4477 17 20C17 20.5523 16.5523 21 16 21H8C7.44772 21 7 20.5523 7 20C7 19.4477 7.44772 19 8 19H11V17ZM4 5H20C20.5523 5 21 5.44772 21 6V14C21 14.5523 20.5523 15 20 15H4C3.44772 15 3 14.5523 3 14V6C3 5.44772 3.44772 5 4 5Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
