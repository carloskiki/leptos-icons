#[cfg(feature = "HiMdSolidChevronDoubleDown")]
use leptos::*;
#[cfg(feature = "HiMdSolidChevronDoubleDown")]
///This icon requires the feature `HiMdSolidChevronDoubleDown` to be enabled.
#[component]
pub fn ChevronDoubleDown(
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
        "M14.7698 4.20938C15.0684 4.49647 15.0777 4.97125 14.7906 5.26983L10.5406 9.76983C10.3992 9.91689 10.204 10 10 10C9.79599 10 9.60078 9.91689 9.45937 9.76983L5.20937 5.26983C4.92228 4.97125 4.93159 4.49647 5.23017 4.20938C5.52875 3.92228 6.00353 3.93159 6.29062 4.23017L10 8.16792L13.7094 4.23017C13.9965 3.93159 14.4713 3.92228 14.7698 4.20938ZM14.7698 10.2094C15.0684 10.4965 15.0777 10.9713 14.7906 11.2698L10.5406 15.7698C10.3992 15.9169 10.204 16 10 16C9.79599 16 9.60078 15.9169 9.45937 15.7698L5.20937 11.2698C4.92228 10.9713 4.93159 10.4965 5.23017 10.2094C5.52875 9.92228 6.00353 9.93159 6.29062 10.2302L10 14.1679L13.7094 10.2302C13.9965 9.93159 14.4713 9.92228 14.7698 10.2094Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
