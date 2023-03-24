#[cfg(feature = "BiSolidFilm")]
use leptos::*;
#[cfg(feature = "BiSolidFilm")]
///This icon requires the feature `BiSolidFilm` to be enabled.
#[component]
pub fn Film(
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
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M19 4v1h-2V3H7v2H5V3H3v18h2v-2h2v2h10v-2h2v2h2V3h-2v1zM5 7h2v2H5V7zm0 4h2v2H5v-2zm0 6v-2h2v2H5zm12 0v-2h2v2h-2zm2-4h-2v-2h2v2zm-2-4V7h2v2h-2z"
        /> < title > { title } < / title > < / svg >
    }
}
