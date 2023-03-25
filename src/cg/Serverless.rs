#[cfg(feature = "CgServerless")]
use leptos::*;
#[cfg(feature = "CgServerless")]
///This icon requires the feature `CgServerless` to be enabled.
#[component]
pub fn Serverless(
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
        "M11.7872 6H5V9H10.6953L11.7872 6Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9.96735 11H5V14H8.87544L9.96735 11Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M11.0038 14L12.0957 11H20V14H11.0038Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8.1475 16H5V19H7.05559L8.1475 16Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9.18394 19L10.2759 16H20V19H9.18394Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12.8236 9L13.9156 6H20V9H12.8236Z" fill =
        "currentColor" /> < title > { title } < / title > < / svg >
    }
}
