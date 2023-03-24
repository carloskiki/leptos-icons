#[cfg(feature = "IoJournalOutline")]
use leptos::*;
#[cfg(feature = "IoJournalOutline")]
///This icon requires the feature `IoJournalOutline` to be enabled.
#[component]
pub fn JournalOutline(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < rect xmlns =
        "http://www.w3.org/2000/svg" x = "96" y = "48" width = "320" height = "416" rx =
        "48" ry = "48" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "320" y1 = "48" x2 = "320" y2 = "464" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:60px" /> < title > {
        title } < / title > < / svg >
    }
}
