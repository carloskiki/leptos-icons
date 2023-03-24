#[cfg(feature = "FiBold")]
use leptos::*;
#[cfg(feature = "FiBold")]
///This icon requires the feature `FiBold` to be enabled.
#[component]
pub fn Bold(
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
        fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap =
        "round" stroke - linejoin = "round" width = { size.clone() } height = { size } >
        < path xmlns = "http://www.w3.org/2000/svg" d =
        "M6 4h8a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M6 12h9a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z" /> <
        title > { title } < / title > < / svg >
    }
}
