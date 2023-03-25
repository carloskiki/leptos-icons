#[cfg(feature = "FiLayout")]
use leptos::*;
#[cfg(feature = "FiLayout")]
///This icon requires the feature `FiLayout` to be enabled.
#[component]
pub fn Layout(
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
        "http://www.w3.org/2000/svg" > < rect xmlns = "http://www.w3.org/2000/svg" x =
        "3" y = "3" width = "18" height = "18" rx = "2" ry = "2" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "3" y1 = "9" x2 = "21" y2 = "9" />< line xmlns
        = "http://www.w3.org/2000/svg" x1 = "9" y1 = "21" x2 = "9" y2 = "9" /> < title >
        { title } < / title > < / svg >
    }
}
