#[cfg(feature = "CgPhotoscan")]
use leptos::*;
#[cfg(feature = "CgPhotoscan")]
///This icon requires the feature `CgPhotoscan` to be enabled.
#[component]
pub fn Photoscan(
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
        "M9 8C9.55228 8 10 7.55228 10 7C10 6.44772 9.55228 6 9 6C8.44772 6 8 6.44772 8 7C8 7.55228 8.44772 8 9 8Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M17 3C18.1046 3 19 3.89543 19 5V19C19 20.1046 18.1046 21 17 21H7C5.89543 21 5 20.1046 5 19V5C5 3.89543 5.89543 3 7 3H17ZM12.4649 5H17V16H7L7 10.4649C7.58835 10.8052 8.27143 11 9 11C11.2091 11 13 9.20914 13 7C13 6.27143 12.8052 5.58835 12.4649 5ZM9 5C7.89543 5 7 5.89543 7 7C7 8.10457 7.89543 9 9 9C10.1046 9 11 8.10457 11 7C11 5.89543 10.1046 5 9 5Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
