#[cfg(feature = "BiSolidKeyboard")]
use leptos::*;
#[cfg(feature = "BiSolidKeyboard")]
///This icon requires the feature `BiSolidKeyboard` to be enabled.
#[component]
pub fn Keyboard(
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
        "M21 5H3a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h18a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2zm-8 2h2v2h-2V7zm0 4h2v2h-2v-2zM9 7h2v2H9V7zm0 4h2v2H9v-2zM5 7h2v2H5V7zm0 4h2v2H5v-2zm12 6H7v-2h10v2zm2-4h-2v-2h2v2zm0-4h-2V7h2v2z"
        /> < title > { title } < / title > < / svg >
    }
}
