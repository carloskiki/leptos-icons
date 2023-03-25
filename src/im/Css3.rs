#[cfg(feature = "ImCss3")]
use leptos::*;
#[cfg(feature = "ImCss3")]
///This icon requires the feature `ImCss3` to be enabled.
#[component]
pub fn Css3(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M2.381 0.758l-0.537 2.686h10.934l-0.342 1.735h-10.94l-0.53 2.686h10.933l-0.61 3.063-4.406 1.46-3.819-1.46 0.261-1.329h-2.686l-0.639 3.224 6.316 2.417 7.281-2.417 2.403-12.066z"
        /> < title > { title } < / title > < / svg >
    }
}
