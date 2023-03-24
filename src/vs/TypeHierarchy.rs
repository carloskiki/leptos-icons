#[cfg(feature = "VsTypeHierarchy")]
use leptos::*;
#[cfg(feature = "VsTypeHierarchy")]
///This icon requires the feature `VsTypeHierarchy` to be enabled.
#[component]
pub fn TypeHierarchy(
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
        "http://www.w3.org/2000/svg" d =
        "M13.5 12h-1.793L10 10.293V6.5L9.5 6H8V4h.5l.5-.5v-2L8.5 1h-2l-.5.5v2l.5.5H7v2H5.5l-.5.5v3.793L3.293 12H1.5l-.5.5v2l.5.5h2l.5-.5v-1.793L5.707 11h3.586L11 12.707V14.5l.5.5h2l.5-.5v-2l-.5-.5zM7 2h1v1H7V2zM6 7h3v3H6V7zm-3 7H2v-1h1v1zm10 0h-1v-1h1v1z"
        /> < title > { title } < / title > < / svg >
    }
}
