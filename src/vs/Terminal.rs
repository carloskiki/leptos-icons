#[cfg(feature = "VsTerminal")]
use leptos::*;
#[cfg(feature = "VsTerminal")]
///This icon requires the feature `VsTerminal` to be enabled.
#[component]
pub fn Terminal(
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
        fill = "currentColor" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M3 1.5L1.5 3v18L3 22.5h18l1.5-1.5V3L21 1.5H3zM3 21V3h18v18H3zm5.656-4.01l1.038 1.061 5.26-5.243v-.912l-5.26-5.26-1.035 1.06 4.59 4.702-4.593 4.592z"
        /> < title > { title } < / title > < / svg >
    }
}
