#[cfg(feature = "CgArrowDownO")]
use leptos::*;
#[cfg(feature = "CgArrowDownO")]
///This icon requires the feature `CgArrowDownO` to be enabled.
#[component]
pub fn ArrowDownO(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M14.8285 12.0259L16.2427 13.4402L12 17.6828L7.7574 13.4402L9.17161 12.0259L11 13.8544V6.31724H13V13.8544L14.8285 12.0259Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M19.7782 19.7782C15.4824 24.0739 8.51759 24.0739 4.22183 19.7782C-0.0739417 15.4824 -0.0739417 8.51759 4.22183 4.22183C8.51759 -0.0739419 15.4824 -0.0739419 19.7782 4.22183C24.0739 8.51759 24.0739 15.4824 19.7782 19.7782ZM18.364 18.364C14.8492 21.8787 9.15076 21.8787 5.63604 18.364C2.12132 14.8492 2.12132 9.15076 5.63604 5.63604C9.15076 2.12132 14.8492 2.12132 18.364 5.63604C21.8787 9.15076 21.8787 14.8492 18.364 18.364Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
