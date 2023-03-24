#[cfg(feature = "BiSolidLandscape")]
use leptos::*;
#[cfg(feature = "BiSolidLandscape")]
///This icon requires the feature `BiSolidLandscape` to be enabled.
#[component]
pub fn Landscape(
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
        width = { size.clone() } height = { size } > < circle xmlns =
        "http://www.w3.org/2000/svg" cx = "6.5" cy = "6.5" r = "2.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "m14 7-5.223 8.487L7 13l-5 7h20z" /> < title > {
        title } < / title > < / svg >
    }
}
