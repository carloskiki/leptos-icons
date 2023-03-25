#[cfg(feature = "CgArrowsMergeAltH")]
use leptos::*;
#[cfg(feature = "CgArrowsMergeAltH")]
///This icon requires the feature `CgArrowsMergeAltH` to be enabled.
#[component]
pub fn ArrowsMergeAltH(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    let style = format!("{} color: {};", style, color);
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M1.5033 6H3.5033V11H7.6749L5.84644 9.17154L7.26066 7.75732L11.5033 12L7.26066 16.2426L5.84644 14.8284L7.67483 13H3.5033V18H1.5033V6Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M20.4967 6H22.4967V18H20.4967V13H16.3251L18.1536 14.8285L16.7393 16.2427L12.4967 12L16.7393 7.75739L18.1536 9.17161L16.3252 11H20.4967V6Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
