#[cfg(feature = "ImFilter")]
use leptos::*;
#[cfg(feature = "ImFilter")]
///This icon requires the feature `ImFilter` to be enabled.
#[component]
pub fn Filter(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" fill
        = "#000000" d =
        "M8 0c-4.418 0-8 1.119-8 2.5v1.5l6 6v5c0 0.552 0.895 1 2 1s2-0.448 2-1v-5l6-6v-1.5c0-1.381-3.582-2.5-8-2.5zM1.475 2.169c0.374-0.213 0.9-0.416 1.52-0.586 1.374-0.376 3.152-0.583 5.005-0.583s3.631 0.207 5.005 0.583c0.62 0.17 1.146 0.372 1.52 0.586 0.247 0.141 0.38 0.26 0.442 0.331-0.062 0.071-0.195 0.19-0.442 0.331-0.374 0.213-0.9 0.416-1.52 0.586-1.374 0.376-3.152 0.583-5.005 0.583s-3.631-0.207-5.005-0.583c-0.62-0.17-1.146-0.372-1.52-0.586-0.247-0.141-0.38-0.26-0.442-0.331 0.062-0.071 0.195-0.19 0.442-0.331z"
        /> < title > { title } < / title > < / svg >
    }
}
