#[cfg(feature = "VsFileSubmodule")]
use leptos::*;
#[cfg(feature = "VsFileSubmodule")]
///This icon requires the feature `VsFileSubmodule` to be enabled.
#[component]
pub fn FileSubmodule(
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
        "M2 11h1V6.99H2V11zm1-5.01V5.5l.5-.5h4.43l.43.25.43.75h5.71l.5.5v8l-.5.5h-11l-.5-.5V12H1.5l-.5-.5v-9l.5-.5h4.42l.44.25.43.75h5.71l.5.5V6l-1-.03V4H6.5l-.43-.25L5.64 3H2v2.99h1zm5.07.76L7.64 6H4v3h3.15l.41-.74L8 8h6V7H8.5l-.43-.25zM7.45 10H4v4h10V9H8.3l-.41.74-.44.26z"
        /> < title > { title } < / title > < / svg >
    }
}
