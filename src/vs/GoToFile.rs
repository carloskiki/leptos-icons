#[cfg(feature = "VsGoToFile")]
use leptos::*;
#[cfg(feature = "VsGoToFile")]
///This icon requires the feature `VsGoToFile` to be enabled.
#[component]
pub fn GoToFile(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M6 5.914l2.06-2.06v-.708L5.915 1l-.707.707.043.043.25.25 1 1h-3a2.5 2.5 0 0 0 0 5H4V7h-.5a1.5 1.5 0 1 1 0-3h3L5.207 5.293 5.914 6 6 5.914zM11 2H8.328l-1-1H12l.71.29 3 3L16 5v9l-1 1H6l-1-1V6.5l1 .847V14h9V6h-4V2zm1 0v3h3l-3-3z"
        /> < title > { title } < / title > < / svg >
    }
}
