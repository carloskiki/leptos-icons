#[cfg(feature = "FaSolidNotEqual")]
use leptos::*;
#[cfg(feature = "FaSolidNotEqual")]
///This icon requires the feature `FaSolidNotEqual` to be enabled.
#[component]
pub fn NotEqual(
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
        stroke_witdh = "0" style = style viewBox = "0 0 448 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M353.8 37.4c14.7 9.8 18.7 29.7 8.9 44.4L321.1 144H384c17.7 0 32 14.3 32 32s-14.3 32-32 32H278.5l-64 96H384c17.7 0 32 14.3 32 32s-14.3 32-32 32H171.8l-65.2 97.7c-9.8 14.7-29.7 18.7-44.4 8.9s-18.7-29.7-8.9-44.4L94.9 368H32c-17.7 0-32-14.3-32-32s14.3-32 32-32H137.5l64-96H32c-17.7 0-32-14.3-32-32s14.3-32 32-32H244.2l65.2-97.7c9.8-14.7 29.7-18.7 44.4-8.9z"
        /> < title > { title } < / title > < / svg >
    }
}
