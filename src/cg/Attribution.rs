#[cfg(feature = "CgAttribution")]
use leptos::*;
#[cfg(feature = "CgAttribution")]
///This icon requires the feature `CgAttribution` to be enabled.
#[component]
pub fn Attribution(
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
        "M6 8C6.74028 8 7.38663 7.5978 7.73244 7H14C15.1046 7 16 7.89543 16 9C16 10.1046 15.1046 11 14 11H10C7.79086 11 6 12.7909 6 15C6 17.2091 7.79086 19 10 19H16.2676C16.6134 19.5978 17.2597 20 18 20C19.1046 20 20 19.1046 20 18C20 16.8954 19.1046 16 18 16C17.2597 16 16.6134 16.4022 16.2676 17H10C8.89543 17 8 16.1046 8 15C8 13.8954 8.89543 13 10 13H14C16.2091 13 18 11.2091 18 9C18 6.79086 16.2091 5 14 5H7.73244C7.38663 4.4022 6.74028 4 6 4C4.89543 4 4 4.89543 4 6C4 7.10457 4.89543 8 6 8Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
