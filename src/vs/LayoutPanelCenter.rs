#[cfg(feature = "VsLayoutPanelCenter")]
use leptos::*;
#[cfg(feature = "VsLayoutPanelCenter")]
///This icon requires the feature `VsLayoutPanelCenter` to be enabled.
#[component]
pub fn LayoutPanelCenter(
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
        "http://www.w3.org/2000/svg" d =
        "M2 1L1 2V14L2 15H14L15 14V2L14 1H2ZM2 14V2H4V14H2ZM5 10V2H11V10H5ZM12 2H14V14H12V2Z"
        /> < title > { title } < / title > < / svg >
    }
}
