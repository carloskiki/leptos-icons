#[cfg(feature = "CgInbox")]
use leptos::*;
#[cfg(feature = "CgInbox")]
///This icon requires the feature `CgInbox` to be enabled.
#[component]
pub fn Inbox(
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
        "M2 5C2 3.34315 3.34315 2 5 2H19C20.6569 2 22 3.34315 22 5V20C22 21.1046 21.1046 22 20 22H8.1477C8.09893 22.0036 8.04968 22.0054 8 22.0054H4C2.89543 22.0054 2 21.1099 2 20.0054V5ZM5 4H19C19.5523 4 20 4.44771 20 5V14H16C14.8954 14 14 14.8954 14 16V17H10V16.0054C10 14.9008 9.10457 14.0054 8 14.0054H4V5C4 4.44772 4.44771 4 5 4Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
