#[cfg(feature = "CgChevronDoubleRightO")]
use leptos::*;
#[cfg(feature = "CgChevronDoubleRightO")]
///This icon requires the feature `CgChevronDoubleRightO` to be enabled.
#[component]
pub fn ChevronDoubleRightO(
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
        "M8.46448 7.75739L7.05026 9.1716L9.87869 12L7.05029 14.8284L8.46451 16.2426L12.7071 12L8.46448 7.75739Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M11.2929 9.1716L12.7071 7.75739L16.9498 12L12.7071 16.2426L11.2929 14.8284L14.1213 12L11.2929 9.1716Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M1 12C1 18.0751 5.92487 23 12 23C18.0751 23 23 18.0751 23 12C23 5.92487 18.0751 1 12 1C5.92487 1 1 5.92487 1 12ZM3 12C3 16.9706 7.02944 21 12 21C16.9706 21 21 16.9706 21 12C21 7.02944 16.9706 3 12 3C7.02944 3 3 7.02944 3 12Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
