#[cfg(feature = "CgMergeVertical")]
use leptos::*;
#[cfg(feature = "CgMergeVertical")]
///This icon requires the feature `CgMergeVertical` to be enabled.
#[component]
pub fn MergeVertical(
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
        "M8.97576 12L4.73312 7.75736L3.31891 9.17157L6.14734 12L3.31891 14.8284L4.73312 16.2426L8.97576 12Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M11.9998 19C11.4475 19 10.9998 18.5523 10.9998 18V6C10.9998 5.44772 11.4475 5 11.9998 5C12.5521 5 12.9998 5.44772 12.9998 6V18C12.9998 18.5523 12.5521 19 11.9998 19Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15.0242 12L19.2669 16.2426L20.6811 14.8284L17.8527 12L20.6811 9.17157L19.2669 7.75736L15.0242 12Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
