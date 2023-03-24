#[cfg(feature = "IoMusicalNote")]
use leptos::*;
#[cfg(feature = "IoMusicalNote")]
///This icon requires the feature `IoMusicalNote` to be enabled.
#[component]
pub fn MusicalNote(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M183.83,480a55.2,55.2,0,0,1-32.36-10.55A56.64,56.64,0,0,1,128,423.58a50.26,50.26,0,0,1,34.14-47.73L213,358.73a16.25,16.25,0,0,0,11-15.49V92a32.1,32.1,0,0,1,24.09-31.15L356.48,32.71A22,22,0,0,1,384,54v57.75a32.09,32.09,0,0,1-24.2,31.19l-91.65,23.13A16.24,16.24,0,0,0,256,181.91V424a48.22,48.22,0,0,1-32.78,45.81l-21.47,7.23A56,56,0,0,1,183.83,480Z"
        /> < title > { title } < / title > < / svg >
    }
}
