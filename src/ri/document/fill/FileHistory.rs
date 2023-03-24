#[cfg(feature = "RiDocumentFillFileHistory")]
use leptos::*;
#[cfg(feature = "RiDocumentFillFileHistory")]
///This icon requires the feature `RiDocumentFillFileHistory` to be enabled.
#[component]
pub fn FileHistory(
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
        "none" d = "M0 0L24 0 24 24 0 24z" />< path d =
        "M16 2l5 4.999v14.01c0 .547-.445.991-.993.991H3.993C3.445 22 3 21.545 3 21.008V2.992C3 2.444 3.445 2 3.993 2H16zm-3 7h-2v6h5v-2h-3V9z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
