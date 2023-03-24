#[cfg(feature = "CgEject")]
use leptos::*;
#[cfg(feature = "CgEject")]
///This icon requires the feature `CgEject` to be enabled.
#[component]
pub fn Eject(
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
        "http://www.w3.org/2000/svg" d =
        "M16.9498 14.3948L18.364 12.9805L12 6.61658L5.63605 12.9805L7.05026 14.3948L12 9.445L16.9498 14.3948Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6.00014 17.3835H18.0001V15.3835H6.00014V17.3835Z" fill = "currentColor" /> <
        title > { title } < / title > < / svg >
    }
}
