#[cfg(feature = "RiHealthFillCapsule")]
use leptos::*;
#[cfg(feature = "RiHealthFillCapsule")]
///This icon requires the feature `RiHealthFillCapsule` to be enabled.
#[component]
pub fn Capsule(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = { size.clone() }
        height = { size } > < g xmlns = "http://www.w3.org/2000/svg" >< path fill =
        "none" d = "M0 0H24V24H0z" />< path d =
        "M19.778 4.222c2.343 2.343 2.343 6.142 0 8.485l-2.122 2.12-4.949 4.951c-2.343 2.343-6.142 2.343-8.485 0-2.343-2.343-2.343-6.142 0-8.485l7.07-7.071c2.344-2.343 6.143-2.343 8.486 0zm-4.95 10.606L9.172 9.172l-3.536 3.535c-1.562 1.562-1.562 4.095 0 5.657 1.562 1.562 4.095 1.562 5.657 0l3.535-3.536z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
