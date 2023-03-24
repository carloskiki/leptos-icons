#[cfg(feature = "ImClearFormatting")]
use leptos::*;
#[cfg(feature = "ImClearFormatting")]
///This icon requires the feature `ImClearFormatting` to be enabled.
#[component]
pub fn ClearFormatting(
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
        "M0 14h9v2h-9zM14 2h-4.727l-2.871 11h-2.067l2.871-11h-4.205v-2h11zM14.528 16l-2.028-2.028-2.028 2.028-0.972-0.972 2.028-2.028-2.028-2.028 0.972-0.972 2.028 2.028 2.028-2.028 0.972 0.972-2.028 2.028 2.028 2.028z"
        /> < title > { title } < / title > < / svg >
    }
}
