#[cfg(feature = "BiRegularLaugh")]
use leptos::*;
#[cfg(feature = "BiRegularLaugh")]
///This icon requires the feature `BiRegularLaugh` to be enabled.
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zm0 18c-4.411 0-8-3.589-8-8s3.589-8 8-8 8 3.589 8 8-3.589 8-8 8z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 18c4 0 5-4 5-4H7s1 4 5 4zm5.555-9.168-1.109-1.664-3 2a1.001 1.001 0 0 0 .108 1.727l4 2 .895-1.789-2.459-1.229 1.565-1.045zm-6.557 1.23a1 1 0 0 0-.443-.894l-3-2-1.11 1.664 1.566 1.044-2.459 1.229.895 1.789 4-2a.998.998 0 0 0 .551-.832z"
        /> < title > { title } < / title > < / svg >
    }
}
