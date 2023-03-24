#[cfg(feature = "FiGitMerge")]
use leptos::*;
#[cfg(feature = "FiGitMerge")]
///This icon requires the feature `FiGitMerge` to be enabled.
#[component]
pub fn GitMerge(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap =
        "round" stroke - linejoin = "round" width = { size.clone() } height = { size } >
        < circle xmlns = "http://www.w3.org/2000/svg" cx = "18" cy = "18" r = "3" /><
        circle xmlns = "http://www.w3.org/2000/svg" cx = "6" cy = "6" r = "3" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M6 21V9a9 9 0 0 0 9 9" /> < title > {
        title } < / title > < / svg >
    }
}
