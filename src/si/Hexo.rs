#[cfg(feature = "SiHexo")]
use leptos::*;
#[cfg(feature = "SiHexo")]
///This icon requires the feature `SiHexo` to be enabled.
#[component]
pub fn Hexo(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M12.02 0L1.596 6.02l-.02 12L11.978 24l10.426-6.02.02-12zm4.828 17.14l-.96.558-.969-.574V12.99H9.081v4.15l-.96.558-.969-.574V6.854l.964-.552.965.563v4.145h5.838V6.86l.965-.552.964.563z"
        /> < title > { title } < / title > < / svg >
    }
}
