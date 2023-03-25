#[cfg(feature = "RiDevelopmentLineGitMerge")]
use leptos::*;
#[cfg(feature = "RiDevelopmentLineGitMerge")]
///This icon requires the feature `RiDevelopmentLineGitMerge` to be enabled.
#[component]
pub fn GitMerge(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M7.105 8.79A3.001 3.001 0 0 0 10 11h4a5.001 5.001 0 0 1 4.927 4.146A3.001 3.001 0 0 1 18 21a3 3 0 0 1-1.105-5.79A3.001 3.001 0 0 0 14 13h-4a4.978 4.978 0 0 1-3-1v3.17a3.001 3.001 0 1 1-2 0V8.83a3.001 3.001 0 1 1 2.105-.04zM6 7a1 1 0 1 0 0-2 1 1 0 0 0 0 2zm0 12a1 1 0 1 0 0-2 1 1 0 0 0 0 2zm12 0a1 1 0 1 0 0-2 1 1 0 0 0 0 2z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
