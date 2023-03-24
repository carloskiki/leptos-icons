#[cfg(feature = "BiSolidHdd")]
use leptos::*;
#[cfg(feature = "BiSolidHdd")]
///This icon requires the feature `BiSolidHdd` to be enabled.
#[component]
pub fn Hdd(
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
        "M20 13H4a2 2 0 0 0-2 2v4a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-4a2 2 0 0 0-2-2zm-4 5h-2v-2h2v2zm4 0h-2v-2h2v2zm.775-7H21l-1.652-7.434A2 2 0 0 0 17.396 2H6.604a2 2 0 0 0-1.952 1.566L3 11h17.775z"
        /> < title > { title } < / title > < / svg >
    }
}
