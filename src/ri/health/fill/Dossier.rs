#[cfg(feature = "RiHealthFillDossier")]
use leptos::*;
#[cfg(feature = "RiHealthFillDossier")]
///This icon requires the feature `RiHealthFillDossier` to be enabled.
#[component]
pub fn Dossier(
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
        "M17 2v2h3c.552 0 1 .448 1 1v16c0 .552-.448 1-1 1H4c-.552 0-1-.448-1-1V5c0-.552.448-1 1-1h3V2h10zm-4 9h-2v2H9v2h1.999L11 17h2l-.001-2H15v-2h-2v-2zm2-7H9v2h6V4z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
