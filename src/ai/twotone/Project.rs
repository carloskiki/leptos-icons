#[cfg(feature = "AiTwotoneProject")]
use leptos::*;
#[cfg(feature = "AiTwotoneProject")]
///This icon requires the feature `AiTwotoneProject` to be enabled.
#[component]
pub fn Project(
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
        stroke_witdh = "0" style = style viewBox = "0 0 1024 1024" width = { size.clone()
        } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" fill = "#333" d
        =
        "M880 112H144c-17.7 0-32 14.3-32 32v736c0 17.7 14.3 32 32 32h736c17.7 0 32-14.3 32-32V144c0-17.7-14.3-32-32-32zm-40 728H184V184h656v656z"
        />< path xmlns = "http://www.w3.org/2000/svg" fill = "#E6E6E6" d =
        "M184 840h656V184H184v656zm472-560c0-4.4 3.6-8 8-8h80c4.4 0 8 3.6 8 8v256c0 4.4-3.6 8-8 8h-80c-4.4 0-8-3.6-8-8V280zm-192 0c0-4.4 3.6-8 8-8h80c4.4 0 8 3.6 8 8v184c0 4.4-3.6 8-8 8h-80c-4.4 0-8-3.6-8-8V280zm-192 0c0-4.4 3.6-8 8-8h80c4.4 0 8 3.6 8 8v464c0 4.4-3.6 8-8 8h-80c-4.4 0-8-3.6-8-8V280z"
        />< path xmlns = "http://www.w3.org/2000/svg" fill = "#333" d =
        "M280 752h80c4.4 0 8-3.6 8-8V280c0-4.4-3.6-8-8-8h-80c-4.4 0-8 3.6-8 8v464c0 4.4 3.6 8 8 8zm192-280h80c4.4 0 8-3.6 8-8V280c0-4.4-3.6-8-8-8h-80c-4.4 0-8 3.6-8 8v184c0 4.4 3.6 8 8 8zm192 72h80c4.4 0 8-3.6 8-8V280c0-4.4-3.6-8-8-8h-80c-4.4 0-8 3.6-8 8v256c0 4.4 3.6 8 8 8z"
        /> < title > { title } < / title > < / svg >
    }
}
