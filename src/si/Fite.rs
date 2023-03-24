#[cfg(feature = "SiFite")]
use leptos::*;
#[cfg(feature = "SiFite")]
///This icon requires the feature `SiFite` to be enabled.
#[component]
pub fn Fite(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M3.52 8.254c-.347 0-.667.08-1.014.213-.427.186-.666.454-.746.8L0 15.748h2.533l.801-3.015h3.28l.4-1.464H3.732l.375-1.362c.027-.133.133-.185.346-.185h3.014l.346-1.467H3.52zm4.826 0l-2 7.492H8.88l2-7.492H8.346zm2.8 0l-.373 1.467h1.84c.16 0 .24.053.24.16v.025l-1.574 5.84h2.534l1.574-5.84c.026-.133.132-.185.345-.185h1.842l.319-1.467h-6.747zm8.534 0c-.347 0-.667.08-1.014.213-.427.186-.666.453-.746.773l-1.467 5.492c0 .027-.027.081-.027.108v.107c0 .267.162.48.455.614.24.106.505.185.799.185H22l.4-1.467h-3.013c-.16 0-.24-.053-.24-.16v-.025l.373-1.387h3.28l.4-1.467h-3.28l.348-1.334c.053-.133.159-.185.345-.185h3.014L24 8.254h-4.32Z"
        /> < title > { title } < / title > < / svg >
    }
}
