#[cfg(feature = "CgSmartHomeBoiler")]
use leptos::*;
#[cfg(feature = "CgSmartHomeBoiler")]
///This icon requires the feature `CgSmartHomeBoiler` to be enabled.
#[component]
pub fn SmartHomeBoiler(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M5 5C5 2.79086 6.79086 1 9 1H15C17.2091 1 19 2.79086 19 5V21H15.144L15.8865 22.9999H13.8865L13.144 21H11.144L11.8865 22.9999H9.88653L9.14397 21H5V5ZM9 3H15C16.1046 3 17 3.89543 17 5V7H7V5C7 3.89543 7.89543 3 9 3ZM7 9H17V19H7V9Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
