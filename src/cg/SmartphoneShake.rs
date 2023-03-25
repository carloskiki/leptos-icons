#[cfg(feature = "CgSmartphoneShake")]
use leptos::*;
#[cfg(feature = "CgSmartphoneShake")]
///This icon requires the feature `CgSmartphoneShake` to be enabled.
#[component]
pub fn SmartphoneShake(
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
        "M13 14H11V16H13V14Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M8 7C8 5.89543 8.89543 5 10 5H14C15.1046 5 16 5.89543 16 7V17C16 18.1046 15.1046 19 14 19H10C8.89543 19 8 18.1046 8 17V7ZM10 7H14V17H10V7Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M18 9H20V15H18V9Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M0 14H2V10H0V14Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M6 15H4V9H6V15Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M24 10H22V14H24V10Z" fill = "currentColor" /> < title > { title } < / title > <
        / svg >
    }
}
