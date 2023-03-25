#[cfg(feature = "FiFeather")]
use leptos::*;
#[cfg(feature = "FiFeather")]
///This icon requires the feature `FiFeather` to be enabled.
#[component]
pub fn Feather(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M20.24 12.24a6 6 0 0 0-8.49-8.49L5 10.5V19h8.5z" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "16" y1 = "8" x2 = "2" y2 = "22" />< line xmlns
        = "http://www.w3.org/2000/svg" x1 = "17.5" y1 = "15" x2 = "9" y2 = "15" /> <
        title > { title } < / title > < / svg >
    }
}
