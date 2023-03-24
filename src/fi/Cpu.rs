#[cfg(feature = "FiCpu")]
use leptos::*;
#[cfg(feature = "FiCpu")]
///This icon requires the feature `FiCpu` to be enabled.
#[component]
pub fn Cpu(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
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
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap =
        "round" stroke - linejoin = "round" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < rect xmlns = "http://www.w3.org/2000/svg" x =
        "4" y = "4" width = "16" height = "16" rx = "2" ry = "2" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "9" y = "9" width = "6" height = "6" />< line
        xmlns = "http://www.w3.org/2000/svg" x1 = "9" y1 = "1" x2 = "9" y2 = "4" />< line
        xmlns = "http://www.w3.org/2000/svg" x1 = "15" y1 = "1" x2 = "15" y2 = "4" /><
        line xmlns = "http://www.w3.org/2000/svg" x1 = "9" y1 = "20" x2 = "9" y2 = "23"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "15" y1 = "20" x2 = "15" y2 =
        "23" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "20" y1 = "9" x2 = "23"
        y2 = "9" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "20" y1 = "14" x2 =
        "23" y2 = "14" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "1" y1 = "9" x2
        = "4" y2 = "9" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "1" y1 = "14"
        x2 = "4" y2 = "14" /> < title > { title } < / title > < / svg >
    }
}
