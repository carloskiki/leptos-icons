#[cfg(feature = "SiCodeclimate")]
use leptos::*;
#[cfg(feature = "SiCodeclimate")]
///This icon requires the feature `SiCodeclimate` to be enabled.
#[component]
pub fn Codeclimate(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M16.125 5.272l-4.511 4.475 2.684 2.659 1.827-1.813 5.19 5.145L24 13.079zM8.13 8.265L0 16.066l2.772 2.662 5.357-5.145 5.357 5.145 2.772-2.662z"
        /> < title > { title } < / title > < / svg >
    }
}
