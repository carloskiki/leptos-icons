#[cfg(feature = "OcLgGitCommit")]
use leptos::*;
#[cfg(feature = "OcLgGitCommit")]
///This icon requires the feature `OcLgGitCommit` to be enabled.
#[component]
pub fn GitCommit(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M16.944 11h4.306a.75.75 0 0 1 0 1.5h-4.306a5.001 5.001 0 0 1-9.888 0H2.75a.75.75 0 0 1 0-1.5h4.306a5.001 5.001 0 0 1 9.888 0Zm-1.444.75a3.5 3.5 0 1 0-7 0 3.5 3.5 0 0 0 7 0Z"
        /> < title > { title } < / title > < / svg >
    }
}
