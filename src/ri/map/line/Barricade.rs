#[cfg(feature = "RiMapLineBarricade")]
use leptos::*;
#[cfg(feature = "RiMapLineBarricade")]
///This icon requires the feature `RiMapLineBarricade` to be enabled.
#[component]
pub fn Barricade(
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
        "M6.493 19h11.014l-.667-3H7.16l-.667 3zm13.063 0H21v2H3v-2h1.444L7.826 3.783A1 1 0 0 1 8.802 3h6.396a1 1 0 0 1 .976.783L19.556 19zM7.604 14h8.792l-.89-4H8.494l-.889 4zm1.334-6h6.124l-.666-3H9.604l-.666 3z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
