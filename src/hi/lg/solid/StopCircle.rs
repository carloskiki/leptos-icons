#[cfg(feature = "HiLgSolidStopCircle")]
use leptos::*;
#[cfg(feature = "HiLgSolidStopCircle")]
///This icon requires the feature `HiLgSolidStopCircle` to be enabled.
#[component]
pub fn StopCircle(
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
        "M2.25 12C2.25 6.61522 6.61522 2.25 12 2.25C17.3848 2.25 21.75 6.61522 21.75 12C21.75 17.3848 17.3848 21.75 12 21.75C6.61522 21.75 2.25 17.3848 2.25 12ZM8.25 9.5625C8.25 8.83763 8.83763 8.25 9.5625 8.25H14.4375C15.1624 8.25 15.75 8.83763 15.75 9.5625V14.4375C15.75 15.1624 15.1624 15.75 14.4375 15.75H9.5625C8.83763 15.75 8.25 15.1624 8.25 14.4375V9.5625Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
