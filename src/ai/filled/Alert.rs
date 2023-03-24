#[cfg(feature = "AiFilledAlert")]
use leptos::*;
#[cfg(feature = "AiFilledAlert")]
///This icon requires the feature `AiFilledAlert` to be enabled.
#[component]
pub fn Alert(
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
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style class = "icon" viewBox = "0 0 1024 1024" width =
        size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M512 244c176.18 0 319 142.82 319 319v233a32 32 0 0 1-32 32H225a32 32 0 0 1-32-32V563c0-176.18 142.82-319 319-319zM484 68h56a8 8 0 0 1 8 8v96a8 8 0 0 1-8 8h-56a8 8 0 0 1-8-8V76a8 8 0 0 1 8-8zM177.25 191.66a8 8 0 0 1 11.32 0l67.88 67.88a8 8 0 0 1 0 11.31l-39.6 39.6a8 8 0 0 1-11.31 0l-67.88-67.88a8 8 0 0 1 0-11.31l39.6-39.6zm669.6 0l39.6 39.6a8 8 0 0 1 0 11.3l-67.88 67.9a8 8 0 0 1-11.32 0l-39.6-39.6a8 8 0 0 1 0-11.32l67.89-67.88a8 8 0 0 1 11.31 0zM192 892h640a32 32 0 0 1 32 32v24a8 8 0 0 1-8 8H168a8 8 0 0 1-8-8v-24a32 32 0 0 1 32-32zm148-317v253h64V575h-64z"
        /> < title > { title } < / title > < / svg >
    }
}
