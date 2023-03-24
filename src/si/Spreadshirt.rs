#[cfg(feature = "SiSpreadshirt")]
use leptos::*;
#[cfg(feature = "SiSpreadshirt")]
///This icon requires the feature `SiSpreadshirt` to be enabled.
#[component]
pub fn Spreadshirt(
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
        "M12 6.306L7.796 2.102 0 9.898l12 12 12-12-7.796-7.796zm0 12L3.592 9.898l4.204-4.204L12 9.898l4.184-4.184 4.204 4.204"
        /> < title > { title } < / title > < / svg >
    }
}
