#[cfg(feature = "SiCapacitor")]
use leptos::*;
#[cfg(feature = "SiCapacitor")]
///This icon requires the feature `SiCapacitor` to be enabled.
#[component]
pub fn Capacitor(
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
        "M24 3.7l-5.766 5.766 5.725 5.736-3.713 3.712L5.073 3.742 8.786.03l5.736 5.726L20.284 0 24 3.7zM.029 8.785l3.713-3.713 15.173 15.173-3.713 3.714-5.732-5.726L3.7 24 0 20.285l5.754-5.764L.029 8.785z"
        /> < title > { title } < / title > < / svg >
    }
}
