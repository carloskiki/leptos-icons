#[cfg(feature = "FaBrandsOrcid")]
use leptos::*;
#[cfg(feature = "FaBrandsOrcid")]
///This icon requires the feature `FaBrandsOrcid` to be enabled.
#[component]
pub fn Orcid(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M294.75 188.19h-45.92V342h47.47c67.62 0 83.12-51.34 83.12-76.91 0-41.64-26.54-76.9-84.67-76.9zM256 8C119 8 8 119 8 256s111 248 248 248 248-111 248-248S393 8 256 8zm-80.79 360.76h-29.84v-207.5h29.84zm-14.92-231.14a19.57 19.57 0 1 1 19.57-19.57 19.64 19.64 0 0 1-19.57 19.57zM300 369h-81V161.26h80.6c76.73 0 110.44 54.83 110.44 103.85C410 318.39 368.38 369 300 369z"
        /> < title > { title } < / title > < / svg >
    }
}
