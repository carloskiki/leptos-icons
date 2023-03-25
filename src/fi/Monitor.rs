#[cfg(feature = "FiMonitor")]
use leptos::*;
#[cfg(feature = "FiMonitor")]
///This icon requires the feature `FiMonitor` to be enabled.
#[component]
pub fn Monitor(
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
        "2" y = "3" width = "20" height = "14" rx = "2" ry = "2" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "8" y1 = "21" x2 = "16" y2 = "21" />< line
        xmlns = "http://www.w3.org/2000/svg" x1 = "12" y1 = "17" x2 = "12" y2 = "21" /> <
        title > { title } < / title > < / svg >
    }
}
