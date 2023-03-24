#[cfg(feature = "ImOmega")]
use leptos::*;
#[cfg(feature = "ImOmega")]
///This icon requires the feature `ImOmega` to be enabled.
#[component]
pub fn Omega(
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
        "M11 14h4l1-2v4h-6v-3.347c2.049-0.883 3.5-3.081 3.5-5.653 0-3.35-2.462-5.973-5.5-5.973s-5.5 2.622-5.5 5.973c0 2.572 1.451 4.77 3.5 5.653v3.347h-6v-4l1 2h4v-0.509c-2.932-1.038-5-3.553-5-6.491 0-3.866 3.582-7 8-7s8 3.134 8 7c0 2.938-2.068 5.452-5 6.491v0.509z"
        /> < title > { title } < / title > < / svg >
    }
}
