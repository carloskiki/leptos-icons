#[cfg(feature = "RiDocumentFillFileSearch")]
use leptos::*;
#[cfg(feature = "RiDocumentFillFileSearch")]
///This icon requires the feature `RiDocumentFillFileSearch` to be enabled.
#[component]
pub fn FileSearch(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = { size.clone() }
        height = { size } > < g xmlns = "http://www.w3.org/2000/svg" >< path fill =
        "none" d = "M0 0h24v24H0z" />< path d =
        "M16 2l5 5v14.008a.993.993 0 0 1-.993.992H3.993A1 1 0 0 1 3 21.008V2.992C3 2.444 3.445 2 3.993 2H16zm-2.471 12.446l2.21 2.21 1.415-1.413-2.21-2.21a4.002 4.002 0 0 0-6.276-4.861 4 4 0 0 0 4.861 6.274zm-.618-2.032a2 2 0 1 1-2.828-2.828 2 2 0 0 1 2.828 2.828z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
