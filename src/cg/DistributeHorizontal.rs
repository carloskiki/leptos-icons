#[cfg(feature = "CgDistributeHorizontal")]
use leptos::*;
#[cfg(feature = "CgDistributeHorizontal")]
///This icon requires the feature `CgDistributeHorizontal` to be enabled.
#[component]
pub fn DistributeHorizontal(
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
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d = "M11 9H13V15H11V9Z" stroke = "currentColor"
        stroke - opacity = "0.5" stroke - width = "2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5 5V19H7V5H5Z" fill = "currentColor" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M17 5V19H19V5H17Z" fill =
        "currentColor" /> < title > { title } < / title > < / svg >
    }
}
