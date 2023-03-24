#[cfg(feature = "TiDropbox")]
use leptos::*;
#[cfg(feature = "TiDropbox")]
///This icon requires the feature `TiDropbox` to be enabled.
#[component]
pub fn Dropbox(
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
        stroke_witdh = "0" style = style version = "1" width = "24" height = "24" viewBox
        = "0 0 24 24" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M3 12.9l5.3 3.5 3.7-3.1-5.3-3.3zM8.3 3.6l-5.3 3.5 3.7 2.9 5.3-3.3zM21 7.1l-5.3-3.5-3.7 3.1 5.3 3.3zM12 13.3l3.7 3.1 5.3-3.5-3.7-2.9zM12 14.5l-3.7 3.1-1.6-1.1v1.2l5.3 3.2 5.3-3.2v-1.2l-1.6 1.1z"
        /> < title > { title } < / title > < / svg >
    }
}
