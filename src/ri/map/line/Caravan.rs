#[cfg(feature = "RiMapLineCaravan")]
use leptos::*;
#[cfg(feature = "RiMapLineCaravan")]
///This icon requires the feature `RiMapLineCaravan` to be enabled.
#[component]
pub fn Caravan(
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
        "none" d = "M0 0L24 0 24 24 0 24z" />< path d =
        "M14.172 3c.53 0 1.039.21 1.414.586l4.828 4.828c.375.375.586.884.586 1.414V17h2v2h-8.126c-.445 1.726-2.01 3-3.874 3-1.864 0-3.43-1.274-3.874-3H3c-.552 0-1-.448-1-1V5c0-1.105.895-2 2-2h10.172zM11 16c-1.105 0-2 .895-2 2s.895 2 2 2 2-.895 2-2-.895-2-2-2zm3.172-11H4v12h3.126c.444-1.725 2.01-3 3.874-3 1.864 0 3.43 1.275 3.874 3H19V9.828L14.172 5zM14 7v6H6V7h8zm-2 2H8v2h4V9z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
