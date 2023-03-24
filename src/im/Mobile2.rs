#[cfg(feature = "ImMobile2")]
use leptos::*;
#[cfg(feature = "ImMobile2")]
///This icon requires the feature `ImMobile2` to be enabled.
#[component]
pub fn Mobile2(
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
        "M12 0h-9c-0.55 0-1 0.45-1 1v14c0 0.55 0.45 1 1 1h9c0.55 0 1-0.45 1-1v-14c0-0.55-0.45-1-1-1zM7.5 15.278c-0.43 0-0.778-0.348-0.778-0.778s0.348-0.778 0.778-0.778 0.778 0.348 0.778 0.778-0.348 0.778-0.778 0.778zM12 13h-9v-11h9v11z"
        /> < title > { title } < / title > < / svg >
    }
}
