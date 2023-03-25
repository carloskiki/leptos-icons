#[cfg(feature = "FiGift")]
use leptos::*;
#[cfg(feature = "FiGift")]
///This icon requires the feature `FiGift` to be enabled.
#[component]
pub fn Gift(
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
        fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap =
        "round" stroke - linejoin = "round" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < polyline xmlns = "http://www.w3.org/2000/svg"
        points = "20 12 20 22 4 22 4 12" />< rect xmlns = "http://www.w3.org/2000/svg" x
        = "2" y = "7" width = "20" height = "5" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "12" y1 = "22" x2 = "12" y2 = "7" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M12 7H7.5a2.5 2.5 0 0 1 0-5C11 2 12 7 12 7z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 7h4.5a2.5 2.5 0 0 0 0-5C13 2 12 7 12 7z" />
        < title > { title } < / title > < / svg >
    }
}
