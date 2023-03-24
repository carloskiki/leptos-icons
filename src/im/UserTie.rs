#[cfg(feature = "ImUserTie")]
use leptos::*;
#[cfg(feature = "ImUserTie")]
///This icon requires the feature `ImUserTie` to be enabled.
#[component]
pub fn UserTie(
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
        "M5 3c0-1.657 1.343-3 3-3s3 1.343 3 3c0 1.657-1.343 3-3 3s-3-1.343-3-3zM12.001 7h-0.553l-3.111 6.316 1.163-5.816-1.5-1.5-1.5 1.5 1.163 5.816-3.111-6.316h-0.554c-1.999 0-1.999 1.344-1.999 3v5h12v-5c0-1.656 0-3-1.999-3z"
        /> < title > { title } < / title > < / svg >
    }
}
