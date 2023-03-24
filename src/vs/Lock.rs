#[cfg(feature = "VsLock")]
use leptos::*;
#[cfg(feature = "VsLock")]
///This icon requires the feature `VsLock` to be enabled.
#[component]
pub fn Lock(
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
        "http://www.w3.org/2000/svg" d =
        "M13 7h-1V5a4 4 0 1 0-8 0v2H3L2 8v6l1 1h10l1-1V8l-1-1zM5 5a3 3 0 1 1 6 0v2H5V5zm8 9H3V8h10v6z"
        /> < title > { title } < / title > < / svg >
    }
}
