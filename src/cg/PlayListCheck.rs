#[cfg(feature = "CgPlayListCheck")]
use leptos::*;
#[cfg(feature = "CgPlayListCheck")]
///This icon requires the feature `CgPlayListCheck` to be enabled.
#[component]
pub fn PlayListCheck(
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
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M15 6H3V8H15V6Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 10H3V12H15V10Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M3 14H11V16H3V14Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M11.9905 15.025L13.4049 13.6106L15.526 15.7321L19.7687 11.4895L21.1829 12.9037L15.526 18.5606L11.9905 15.025Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
