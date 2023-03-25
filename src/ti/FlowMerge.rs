#[cfg(feature = "TiFlowMerge")]
use leptos::*;
#[cfg(feature = "TiFlowMerge")]
///This icon requires the feature `TiFlowMerge` to be enabled.
#[component]
pub fn FlowMerge(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M18 16.184v-1.851c0-1.93-1.57-3.5-3.5-3.5-.827 0-1.5-.673-1.5-1.5v-1.517c1.161-.415 2-1.514 2-2.816 0-1.654-1.346-3-3-3s-3 1.346-3 3c0 1.302.839 2.401 2 2.815v1.518c0 .827-.673 1.5-1.5 1.5-1.93 0-3.5 1.57-3.5 3.5v1.851c-1.161.415-2 1.514-2 2.816 0 1.654 1.346 3 3 3s3-1.346 3-3c0-1.302-.839-2.401-2-2.816v-1.851c0-.827.673-1.5 1.5-1.5.979 0 1.864-.407 2.5-1.058.636.651 1.521 1.058 2.5 1.058.827 0 1.5.673 1.5 1.5v1.851c-1.161.415-2 1.514-2 2.816 0 1.654 1.346 3 3 3s3-1.346 3-3c0-1.302-.839-2.401-2-2.816zm-11 3.816c-.552 0-1-.449-1-1s.448-1 1-1 1 .449 1 1-.448 1-1 1zm5-16c.552 0 1 .449 1 1s-.448 1-1 1-1-.449-1-1 .448-1 1-1zm5 16c-.552 0-1-.449-1-1s.448-1 1-1 1 .449 1 1-.448 1-1 1z"
        /> < title > { title } < / title > < / svg >
    }
}
