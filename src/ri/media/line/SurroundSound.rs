#[cfg(feature = "RiMediaLineSurroundSound")]
use leptos::*;
#[cfg(feature = "RiMediaLineSurroundSound")]
///This icon requires the feature `RiMediaLineSurroundSound` to be enabled.
#[component]
pub fn SurroundSound(
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
        "M4 5v14h16V5H4zM3 3h18a1 1 0 0 1 1 1v16a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1zm4.05 4.121l1.414 1.415A4.984 4.984 0 0 0 7 12.07c0 1.38.56 2.63 1.464 3.536L7.05 17.02A6.978 6.978 0 0 1 5 12.07c0-1.933.784-3.683 2.05-4.95zm9.9 0a6.978 6.978 0 0 1 2.05 4.95 6.978 6.978 0 0 1-2.05 4.95l-1.414-1.414A4.984 4.984 0 0 0 17 12.07c0-1.38-.56-2.63-1.464-3.535L16.95 7.12zM12 13.071a1 1 0 1 0 0-2 1 1 0 0 0 0 2zm0 2a3 3 0 1 1 0-6 3 3 0 0 1 0 6z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
