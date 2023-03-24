#[cfg(feature = "HiMdSolidChevronUp")]
use leptos::*;
#[cfg(feature = "HiMdSolidChevronUp")]
///This icon requires the feature `HiMdSolidChevronUp` to be enabled.
#[component]
pub fn ChevronUp(
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
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M14.7698 12.7906C14.4713 13.0777 13.9965 13.0684 13.7094 12.7698L10 8.83208L6.29062 12.7698C6.00353 13.0684 5.52875 13.0777 5.23017 12.7906C4.93159 12.5035 4.92228 12.0287 5.20937 11.7302L9.45937 7.23017C9.60078 7.08311 9.79599 7 10 7C10.204 7 10.3992 7.08311 10.5406 7.23017L14.7906 11.7302C15.0777 12.0287 15.0684 12.5035 14.7698 12.7906Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
