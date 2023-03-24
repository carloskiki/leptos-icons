#[cfg(feature = "RiHealthLineFlask")]
use leptos::*;
#[cfg(feature = "RiHealthLineFlask")]
///This icon requires the feature `RiHealthLineFlask` to be enabled.
#[component]
pub fn Flask(
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
        "M16 2v2h-1v3.243c0 1.158.251 2.301.736 3.352l4.282 9.276c.347.753.018 1.644-.734 1.99-.197.092-.411.139-.628.139H5.344c-.828 0-1.5-.672-1.5-1.5 0-.217.047-.432.138-.629l4.282-9.276C8.749 9.545 9 8.401 9 7.243V4H8V2h8zm-2.612 8.001h-2.776c-.104.363-.23.721-.374 1.071l-.158.361L6.125 20h11.749l-3.954-8.567c-.214-.464-.392-.943-.532-1.432zM11 7.243c0 .253-.01.506-.029.758h2.058c-.01-.121-.016-.242-.021-.364L13 7.243V4h-2v3.243z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
