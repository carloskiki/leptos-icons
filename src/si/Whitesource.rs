#[cfg(feature = "SiWhitesource")]
use leptos::*;
#[cfg(feature = "SiWhitesource")]
///This icon requires the feature `SiWhitesource` to be enabled.
#[component]
pub fn Whitesource(
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
        "M12 0C5.373 0 0 5.372 0 12c0 6.627 5.373 12 12 12 6.628 0 12-5.373 12-12 0-6.628-5.372-12-12-12zm-1.58 4.916h3.167v5.268h.657V5.818c2.971 1.432 2.554 5.28 2.586 5.136h.718v1.014H6.452v-1.014h.718s-.687-3.559 2.587-5.136v4.366h.662zm-3.968 8.002h3.18V13.464A2.376 2.376 0 0012 15.833a2.376 2.376 0 002.369-2.369V12.918h3.18v2.066h-1.292l-.416.928.872.768-1.596 1.697-.911-.919-.988.424v1.202h-2.436v-1.202l-.987-.424-.912.919-1.595-1.697.871-.768-.415-.928H6.452z"
        /> < title > { title } < / title > < / svg >
    }
}
