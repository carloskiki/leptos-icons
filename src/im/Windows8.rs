#[cfg(feature = "ImWindows8")]
use leptos::*;
#[cfg(feature = "ImWindows8")]
///This icon requires the feature `ImWindows8` to be enabled.
#[component]
pub fn Windows8(
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
        "M0.005 8l-0.005-4.876 6-0.815v5.691zM7 2.164l7.998-1.164v7h-7.998zM15 9l-0.002 7-7.998-1.125v-5.875zM6 14.747l-5.995-0.822-0-4.926h5.995z"
        /> < title > { title } < / title > < / svg >
    }
}
