#[cfg(feature = "BiSolidMessageAlt")]
use leptos::*;
#[cfg(feature = "BiSolidMessageAlt")]
///This icon requires the feature `BiSolidMessageAlt` to be enabled.
#[component]
pub fn MessageAlt(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M18.999 2h-14c-1.103 0-2 .897-2 2v12c0 1.103.897 2 2 2h3.5l3.5 4 3.5-4h3.5c1.103 0 2-.897 2-2V4c0-1.103-.897-2-2-2z"
        /> < title > { title } < / title > < / svg >
    }
}
