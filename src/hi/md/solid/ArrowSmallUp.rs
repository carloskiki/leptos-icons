#[cfg(feature = "HiMdSolidArrowSmallUp")]
use leptos::*;
#[cfg(feature = "HiMdSolidArrowSmallUp")]
///This icon requires the feature `HiMdSolidArrowSmallUp` to be enabled.
#[component]
pub fn ArrowSmallUp(
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
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M10 15C9.58579 15 9.25 14.6642 9.25 14.25L9.25 7.61208L7.29063 9.76983C7.00353 10.0684 6.52875 10.0777 6.23017 9.79062C5.93159 9.50353 5.92228 9.02875 6.20938 8.73017L9.45938 5.23017C9.60078 5.08311 9.79599 5 10 5C10.204 5 10.3992 5.08311 10.5406 5.23017L13.7906 8.73017C14.0777 9.02875 14.0684 9.50353 13.7698 9.79062C13.4713 10.0777 12.9965 10.0684 12.7094 9.76983L10.75 7.61208V14.25C10.75 14.6642 10.4142 15 10 15Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
