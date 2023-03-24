#[cfg(feature = "BiSolidBackpack")]
use leptos::*;
#[cfg(feature = "BiSolidBackpack")]
///This icon requires the feature `BiSolidBackpack` to be enabled.
#[component]
pub fn Backpack(
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
        "M16 15a1 1 0 0 0-1-1H9c-.551 0-1 .448-1 1v2h8v-2zm-8 4h8v3H8z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M21 12c0-2.967-2.167-5.432-5-5.91V5c0-1.654-1.346-3-3-3h-2C9.346 2 8 3.346 8 5v1.09C5.167 6.568 3 9.033 3 12v8c0 1.103.897 2 2 2h1v-7c0-1.654 1.346-3 3-3h6c1.654 0 3 1.346 3 3v7h1c1.103 0 2-.897 2-2v-8zM10 5c0-.552.449-1 1-1h2a1 1 0 0 1 1 1v1h-4V5z"
        /> < title > { title } < / title > < / svg >
    }
}
