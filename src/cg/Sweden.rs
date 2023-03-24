#[cfg(feature = "CgSweden")]
use leptos::*;
#[cfg(feature = "CgSweden")]
///This icon requires the feature `CgSweden` to be enabled.
#[component]
pub fn Sweden(
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
        "http://www.w3.org/2000/svg" d = "M23 4H10V11H23V4Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M23 13V20H10V13H23Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 13V20H1V13H8Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M1 11V4H8V11H1Z" fill = "currentColor" /> <
        title > { title } < / title > < / svg >
    }
}
