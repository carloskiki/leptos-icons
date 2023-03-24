#[cfg(feature = "RiMapFillPinDistance")]
use leptos::*;
#[cfg(feature = "RiMapFillPinDistance")]
///This icon requires the feature `RiMapFillPinDistance` to be enabled.
#[component]
pub fn PinDistance(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = { size.clone() }
        height = { size } > < g xmlns = "http://www.w3.org/2000/svg" >< path fill =
        "none" d = "M0 0h24v24H0z" />< path d =
        "M11.39 10.39L7.5 14.277 3.61 10.39a5.5 5.5 0 1 1 7.78 0zM7.5 8.5a2 2 0 1 0 0-4 2 2 0 0 0 0 4zm12.89 10.89l-3.89 3.888-3.89-3.889a5.5 5.5 0 1 1 7.78 0zM16.5 17.5a2 2 0 1 0 0-4 2 2 0 0 0 0 4z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
