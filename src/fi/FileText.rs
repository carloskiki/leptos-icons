#[cfg(feature = "FiFileText")]
use leptos::*;
#[cfg(feature = "FiFileText")]
///This icon requires the feature `FiFileText` to be enabled.
#[component]
pub fn FileText(
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
        "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />< polyline xmlns =
        "http://www.w3.org/2000/svg" points = "14 2 14 8 20 8" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "16" y1 = "13" x2 = "8" y2 = "13" />< line
        xmlns = "http://www.w3.org/2000/svg" x1 = "16" y1 = "17" x2 = "8" y2 = "17" /><
        polyline xmlns = "http://www.w3.org/2000/svg" points = "10 9 9 9 8 9" /> < title
        > { title } < / title > < / svg >
    }
}
