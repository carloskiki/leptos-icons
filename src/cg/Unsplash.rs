#[cfg(feature = "CgUnsplash")]
use leptos::*;
#[cfg(feature = "CgUnsplash")]
///This icon requires the feature `CgUnsplash` to be enabled.
#[component]
pub fn Unsplash(
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
        "M15 4.5H9V8.5H15V4.5Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 10.5H9V14.5H15V10.5H20V19.5H4V10.5Z" fill =
        "currentColor" /> < title > { title } < / title > < / svg >
    }
}
