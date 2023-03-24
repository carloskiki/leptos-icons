#[cfg(feature = "SiFunimation")]
use leptos::*;
#[cfg(feature = "SiFunimation")]
///This icon requires the feature `SiFunimation` to be enabled.
#[component]
pub fn Funimation(
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
        "M12 0a12 12 0 1 0 12 12A12.001 12.001 0 0 0 12 0ZM7.428 16.06h9.188s-.449 3.278-4.601 3.278c-4.15 0-4.587-3.278-4.587-3.278z"
        /> < title > { title } < / title > < / svg >
    }
}
