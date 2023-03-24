#[cfg(feature = "IoPrintSharp")]
use leptos::*;
#[cfg(feature = "IoPrintSharp")]
///This icon requires the feature `IoPrintSharp` to be enabled.
#[component]
pub fn PrintSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d = "M400,96V56a8,8,0,0,0-8-8H120a8,8,0,0,0-8,8V96"
        />< rect xmlns = "http://www.w3.org/2000/svg" x = "152" y = "264" width = "208"
        height = "160" rx = "4" ry = "4" style = "fill:none" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "152" y = "264" width = "208" height = "160" rx
        = "4" ry = "4" style = "fill:none" />< path xmlns = "http://www.w3.org/2000/svg"
        d =
        "M408,112H104a56,56,0,0,0-56,56V376a8,8,0,0,0,8,8h56v72a8,8,0,0,0,8,8H392a8,8,0,0,0,8-8V384h56a8,8,0,0,0,8-8V168A56,56,0,0,0,408,112ZM360,420a4,4,0,0,1-4,4H156a4,4,0,0,1-4-4V268a4,4,0,0,1,4-4H356a4,4,0,0,1,4,4ZM394,207.92a24,24,0,1,1,22-22A24,24,0,0,1,394,207.92Z"
        /> < title > { title } < / title > < / svg >
    }
}
