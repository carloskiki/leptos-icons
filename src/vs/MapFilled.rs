#[cfg(feature = "VsMapFilled")]
use leptos::*;
#[cfg(feature = "VsMapFilled")]
///This icon requires the feature `VsMapFilled` to be enabled.
#[component]
pub fn MapFilled(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d = "M2 5.5V13L5.5 10.8125V3.3125L2 5.5Z" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M9.5 12.6875V5.1875L6.5 3.3125V10.8125L9.5 12.6875Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10.5 12.6875V5.1875L14 3V10.5L10.5 12.6875Z"
        /> < title > { title } < / title > < / svg >
    }
}
