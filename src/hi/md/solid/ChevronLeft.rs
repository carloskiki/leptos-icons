#[cfg(feature = "HiMdSolidChevronLeft")]
use leptos::*;
#[cfg(feature = "HiMdSolidChevronLeft")]
///This icon requires the feature `HiMdSolidChevronLeft` to be enabled.
#[component]
pub fn ChevronLeft(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M12.7906 5.23017C13.0777 5.52875 13.0684 6.00353 12.7698 6.29063L8.83208 10L12.7698 13.7094C13.0684 13.9965 13.0777 14.4713 12.7906 14.7698C12.5035 15.0684 12.0287 15.0777 11.7302 14.7906L7.23017 10.5406C7.08311 10.3992 7 10.204 7 10C7 9.79599 7.08311 9.60078 7.23017 9.45938L11.7302 5.20938C12.0287 4.92228 12.5035 4.93159 12.7906 5.23017Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
