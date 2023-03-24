#[cfg(feature = "OcSmGoal")]
use leptos::*;
#[cfg(feature = "OcSmGoal")]
///This icon requires the feature `OcSmGoal` to be enabled.
#[component]
pub fn Goal(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M13.637 2.363h-.001l1.676.335c.09.018.164.084.19.173a.25.25 0 0 1-.062.249l-1.373 1.374a.876.876 0 0 1-.619.256H12.31L9.45 7.611A1.5 1.5 0 1 1 6.5 8a1.501 1.501 0 0 1 1.889-1.449l2.861-2.862V2.552c0-.232.092-.455.256-.619L12.88.559a.25.25 0 0 1 .249-.062c.089.026.155.1.173.19Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M2 8a6 6 0 1 0 11.769-1.656.751.751 0 1 1 1.442-.413 7.502 7.502 0 0 1-12.513 7.371A7.501 7.501 0 0 1 10.069.789a.75.75 0 0 1-.413 1.442A6.001 6.001 0 0 0 2 8Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5 8a3.002 3.002 0 0 0 4.699 2.476 3 3 0 0 0 1.28-2.827.748.748 0 0 1 1.045-.782.75.75 0 0 1 .445.61A4.5 4.5 0 1 1 8.516 3.53a.75.75 0 1 1-.17 1.49A3 3 0 0 0 5 8Z"
        /> < title > { title } < / title > < / svg >
    }
}
