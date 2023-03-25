#[cfg(feature = "CgPlayForwards")]
use leptos::*;
#[cfg(feature = "CgPlayForwards")]
///This icon requires the feature `CgPlayForwards` to be enabled.
#[component]
pub fn PlayForwards(
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
        "M21.0023 17H18.0023V7H21.0023V17Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17.0023 12L10 17V7L17.0023 12Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M2 17L9.00232 12L2 7V17Z" fill = "currentColor" /> < title > { title } < / title
        > < / svg >
    }
}
