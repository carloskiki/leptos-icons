#[cfg(feature = "CgDolby")]
use leptos::*;
#[cfg(feature = "CgDolby")]
///This icon requires the feature `CgDolby` to be enabled.
#[component]
pub fn Dolby(
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
        "M0 4V20H24V4H0ZM10 12C10 9.79086 8.20914 8 6 8H4V16H6C8.20914 16 10 14.2091 10 12ZM18 16H20V8H18C15.7909 8 14 9.79086 14 12C14 14.2091 15.7909 16 18 16Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
