#[cfg(feature = "CgArrowLongLeftE")]
use leptos::*;
#[cfg(feature = "CgArrowLongLeftE")]
///This icon requires the feature `CgArrowLongLeftE` to be enabled.
#[component]
pub fn ArrowLongLeftE(
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
        "M5.26323 7.75737L1.01343 11.9928L5.24893 16.2426L6.66553 14.8308L4.85123 13.0104L16.9685 13.0519L16.9596 15.0507L22.9595 15.0777L22.9865 9.07773L16.9865 9.05079L16.9775 11.0519L4.83252 11.0103L6.67505 9.17397L5.26323 7.75737ZM20.9775 11.0687L20.9685 13.0687L18.9685 13.0597L18.9775 11.0597L20.9775 11.0687Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
