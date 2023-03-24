#[cfg(feature = "BiSolidLaugh")]
use leptos::*;
#[cfg(feature = "BiSolidLaugh")]
///This icon requires the feature `BiSolidLaugh` to be enabled.
#[component]
pub fn Laugh(
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
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zm-6.447 9.105 2.459-1.229-1.567-1.044 1.109-1.664 3 2a1 1 0 0 1-.108 1.727l-4 2-.893-1.79zM12 18c-4 0-5-4-5-4h10s-1 4-5 4zm5.553-5.105-4-2a1 1 0 0 1-.108-1.727l3-2 1.109 1.664-1.566 1.044 2.459 1.229-.894 1.79z"
        /> < title > { title } < / title > < / svg >
    }
}
