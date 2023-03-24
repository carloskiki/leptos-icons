#[cfg(feature = "HiMdSolidArrowSmallDown")]
use leptos::*;
#[cfg(feature = "HiMdSolidArrowSmallDown")]
///This icon requires the feature `HiMdSolidArrowSmallDown` to be enabled.
#[component]
pub fn ArrowSmallDown(
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
        "M10 5C10.4142 5 10.75 5.33579 10.75 5.75V12.3879L12.7094 10.2302C12.9965 9.93159 13.4713 9.92228 13.7698 10.2094C14.0684 10.4965 14.0777 10.9713 13.7906 11.2698L10.5406 14.7698C10.3992 14.9169 10.204 15 10 15C9.79599 15 9.60078 14.9169 9.45938 14.7698L6.20938 11.2698C5.92228 10.9713 5.93159 10.4965 6.23017 10.2094C6.52875 9.92228 7.00353 9.93159 7.29063 10.2302L9.25 12.3879V5.75C9.25 5.33579 9.58579 5 10 5Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
