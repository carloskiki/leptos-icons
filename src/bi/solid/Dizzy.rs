#[cfg(feature = "BiSolidDizzy")]
use leptos::*;
#[cfg(feature = "BiSolidDizzy")]
///This icon requires the feature `BiSolidDizzy` to be enabled.
#[component]
pub fn Dizzy(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zM8 12.414l-1.293 1.293-1.414-1.414L6.586 11 5.293 9.707l1.414-1.414L8 9.586l1.293-1.293 1.414 1.414L9.414 11l1.293 1.293-1.414 1.414L8 12.414zM14 18h-4v-2h4v2zm4.707-5.707-1.414 1.414L16 12.414l-1.293 1.293-1.414-1.414L14.586 11l-1.293-1.293 1.414-1.414L16 9.586l1.293-1.293 1.414 1.414L17.414 11l1.293 1.293z"
        /> < title > { title } < / title > < / svg >
    }
}
