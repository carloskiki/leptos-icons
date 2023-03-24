#[cfg(feature = "ImCopy")]
use leptos::*;
#[cfg(feature = "ImCopy")]
///This icon requires the feature `ImCopy` to be enabled.
#[component]
pub fn Copy(
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
        "M10 4v-4h-7l-3 3v9h6v4h10v-12h-6zM3 1.414v1.586h-1.586l1.586-1.586zM1 11v-7h3v-3h5v3l-3 3v4h-5zM9 5.414v1.586h-1.586l1.586-1.586zM15 15h-8v-7h3v-3h5v10z"
        /> < title > { title } < / title > < / svg >
    }
}
