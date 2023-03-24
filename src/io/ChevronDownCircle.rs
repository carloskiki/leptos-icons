#[cfg(feature = "IoChevronDownCircle")]
use leptos::*;
#[cfg(feature = "IoChevronDownCircle")]
///This icon requires the feature `IoChevronDownCircle` to be enabled.
#[component]
pub fn ChevronDownCircle(
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
        "M464,256c0-114.87-93.13-208-208-208S48,141.13,48,256s93.13,208,208,208S464,370.87,464,256ZM363.31,227.31l-96,96a16,16,0,0,1-22.62,0l-96-96a16,16,0,0,1,22.62-22.62L256,289.37l84.69-84.68a16,16,0,0,1,22.62,22.62Z"
        /> < title > { title } < / title > < / svg >
    }
}
