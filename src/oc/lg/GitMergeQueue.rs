#[cfg(feature = "OcLgGitMergeQueue")]
use leptos::*;
#[cfg(feature = "OcLgGitMergeQueue")]
///This icon requires the feature `OcLgGitMergeQueue` to be enabled.
#[component]
pub fn GitMergeQueue(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M5.75 6.5a1.75 1.75 0 1 1 .001-3.501A1.75 1.75 0 0 1 5.75 6.5ZM9.5 8.75a1.75 1.75 0 1 1 3.501.001A1.75 1.75 0 0 1 9.5 8.75ZM5.75 22.5a3.25 3.25 0 0 1-.745-6.414A.81.81 0 0 1 5 16v-5a.75.75 0 0 1 1.5 0v5a.81.81 0 0 1-.005.086A3.252 3.252 0 0 1 5.75 22.5ZM4 19.25a1.75 1.75 0 1 0 3.501-.001A1.75 1.75 0 0 0 4 19.25Zm11-6.5a3.25 3.25 0 1 1 6.5 0 3.25 3.25 0 0 1-6.5 0Zm3.25 1.75a1.75 1.75 0 1 0 0-3.5 1.75 1.75 0 0 0 0 3.5Z"
        /> < title > { title } < / title > < / svg >
    }
}
