#[cfg(feature = "HiMdSolidViewColumns")]
use leptos::*;
#[cfg(feature = "HiMdSolidViewColumns")]
///This icon requires the feature `HiMdSolidViewColumns` to be enabled.
#[component]
pub fn ViewColumns(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M14 17H16.75C17.9926 17 19 15.9926 19 14.75V5.25C19 4.00736 17.9926 3 16.75 3H14V17Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12.5 3H7.5V17H12.5V3Z" fill = "#0F172A" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M3.25 3H6V17H3.25C2.00736 17 1 15.9926 1 14.75V5.25C1 4.00736 2.00736 3 3.25 3Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
