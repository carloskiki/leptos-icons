#[cfg(feature = "BiSolidHappyAlt")]
use leptos::*;
#[cfg(feature = "BiSolidHappyAlt")]
///This icon requires the feature `BiSolidHappyAlt` to be enabled.
#[component]
pub fn HappyAlt(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zm3.493 6a1.494 1.494 0 1 1-.001 2.987A1.494 1.494 0 0 1 15.493 8zM8.5 8a1.5 1.5 0 1 1-.001 3.001A1.5 1.5 0 0 1 8.5 8zM12 18c-5 0-6-5-6-5h12s-1 5-6 5z"
        /> < title > { title } < / title > < / svg >
    }
}
