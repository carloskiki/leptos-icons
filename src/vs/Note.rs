#[cfg(feature = "VsNote")]
use leptos::*;
#[cfg(feature = "VsNote")]
///This icon requires the feature `VsNote` to be enabled.
#[component]
pub fn Note(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M1.5 2h13l.5.5v10l-.5.5h-13l-.5-.5v-10l.5-.5zM2 3v9h12V3H2zm2 2h8v1H4V5zm6 2H4v1h6V7zM4 9h4v1H4V9z"
        /> < title > { title } < / title > < / svg >
    }
}
