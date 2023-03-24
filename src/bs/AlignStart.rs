#[cfg(feature = "BsAlignStart")]
use leptos::*;
#[cfg(feature = "BsAlignStart")]
///This icon requires the feature `BsAlignStart` to be enabled.
#[component]
pub fn AlignStart(
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
        stroke_witdh = "0" style = style width = "16" height = "16" fill = "currentColor"
        class = "bi bi-align-start" viewBox = "0 0 16 16" width = { size.clone() } height
        = { size } > < path xmlns = "http://www.w3.org/2000/svg" fill - rule = "evenodd"
        d = "M1.5 1a.5.5 0 0 1 .5.5v13a.5.5 0 0 1-1 0v-13a.5.5 0 0 1 .5-.5z" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M3 7a1 1 0 0 1 1-1h10a1 1 0 0 1 1 1v2a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1V7z" /> <
        title > { title } < / title > < / svg >
    }
}
