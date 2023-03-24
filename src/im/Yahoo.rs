#[cfg(feature = "ImYahoo")]
use leptos::*;
#[cfg(feature = "ImYahoo")]
///This icon requires the feature `ImYahoo` to be enabled.
#[component]
pub fn Yahoo(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" fill
        = "#000000" d =
        "M8.878 9.203v0c1.759-3.088 4.666-8.125 5.463-9.203-0.35 0.234-0.887 0.353-1.381 0.466l-0.747-0.466c-0.6 1.119-2.813 4.734-4.222 7.050-1.428-2.366-3.119-5.097-4.222-7.050-0.875 0.188-1.237 0.197-2.109 0v0 0c0 0 0 0 0 0v0c1.731 2.606 4.503 7.572 5.447 9.203v0l-0.128 6.797 1.013-0.466v-0.012l1.012 0.478-0.125-6.797z"
        /> < title > { title } < / title > < / svg >
    }
}
