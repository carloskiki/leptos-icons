#[cfg(feature = "OcSmGitCompare")]
use leptos::*;
#[cfg(feature = "OcSmGitCompare")]
///This icon requires the feature `OcSmGitCompare` to be enabled.
#[component]
pub fn GitCompare(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M9.573.677A.25.25 0 0 1 10 .854V2.5h1A2.5 2.5 0 0 1 13.5 5v5.628a2.251 2.251 0 1 1-1.5 0V5a1 1 0 0 0-1-1h-1v1.646a.25.25 0 0 1-.427.177L7.177 3.427a.25.25 0 0 1 0-.354ZM6 12v-1.646a.25.25 0 0 1 .427-.177l2.396 2.396a.25.25 0 0 1 0 .354l-2.396 2.396A.25.25 0 0 1 6 15.146V13.5H5A2.5 2.5 0 0 1 2.5 11V5.372a2.25 2.25 0 1 1 1.5 0V11a1 1 0 0 0 1 1ZM4 3.25a.75.75 0 1 0-1.5 0 .75.75 0 0 0 1.5 0ZM12.75 12a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5Z"
        /> < title > { title } < / title > < / svg >
    }
}
