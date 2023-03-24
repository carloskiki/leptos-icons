#[cfg(feature = "CgGlassAlt")]
use leptos::*;
#[cfg(feature = "CgGlassAlt")]
///This icon requires the feature `CgGlassAlt` to be enabled.
#[component]
pub fn GlassAlt(
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
        "M5 2H19L17.3602 18.398C17.1557 20.4428 15.4351 22 13.38 22H10.62C8.56494 22 6.84428 20.4428 6.6398 18.398L5 2ZM7.50998 7L7.20998 4H16.79L16.49 7H7.50998ZM7.70998 9L8.62988 18.199C8.73212 19.2214 9.59245 20 10.62 20H13.38C14.4076 20 15.2679 19.2214 15.3701 18.199L16.29 9H7.70998Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
