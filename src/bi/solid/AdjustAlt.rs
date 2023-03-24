#[cfg(feature = "BiSolidAdjustAlt")]
use leptos::*;
#[cfg(feature = "BiSolidAdjustAlt")]
///This icon requires the feature `BiSolidAdjustAlt` to be enabled.
#[component]
pub fn AdjustAlt(
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
        "M19.071 19.071c3.833-3.833 3.833-10.31 0-14.143s-10.31-3.833-14.143 0-3.833 10.31 0 14.143 10.31 3.833 14.143 0zM7.051 7.051c2.706-2.707 7.191-2.708 9.898 0l-9.898 9.898c-2.708-2.707-2.71-7.19 0-9.898z"
        /> < title > { title } < / title > < / svg >
    }
}
