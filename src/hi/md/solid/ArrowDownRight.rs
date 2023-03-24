#[cfg(feature = "HiMdSolidArrowDownRight")]
use leptos::*;
#[cfg(feature = "HiMdSolidArrowDownRight")]
///This icon requires the feature `HiMdSolidArrowDownRight` to be enabled.
#[component]
pub fn ArrowDownRight(
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
        "M6.28033 5.21967C5.98744 4.92678 5.51256 4.92678 5.21967 5.21967C4.92678 5.51256 4.92678 5.98744 5.21967 6.28033L12.4393 13.5H6.75C6.33579 13.5 6 13.8358 6 14.25C6 14.6642 6.33579 15 6.75 15H14.25C14.3517 15 14.4487 14.9798 14.5371 14.9431C14.6235 14.9073 14.7047 14.8547 14.7754 14.7852C14.7787 14.782 14.782 14.7787 14.7852 14.7754C14.8547 14.7047 14.9073 14.6235 14.9431 14.5371C14.9798 14.4487 15 14.3517 15 14.25V6.75C15 6.33579 14.6642 6 14.25 6C13.8358 6 13.5 6.33579 13.5 6.75V12.4393L6.28033 5.21967Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
