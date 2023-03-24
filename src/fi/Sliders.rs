#[cfg(feature = "FiSliders")]
use leptos::*;
#[cfg(feature = "FiSliders")]
///This icon requires the feature `FiSliders` to be enabled.
#[component]
pub fn Sliders(
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
        "http://www.w3.org/2000/svg" > < line xmlns = "http://www.w3.org/2000/svg" x1 =
        "4" y1 = "21" x2 = "4" y2 = "14" />< line xmlns = "http://www.w3.org/2000/svg" x1
        = "4" y1 = "10" x2 = "4" y2 = "3" />< line xmlns = "http://www.w3.org/2000/svg"
        x1 = "12" y1 = "21" x2 = "12" y2 = "12" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "12" y1 = "8" x2 = "12" y2 = "3" />< line xmlns
        = "http://www.w3.org/2000/svg" x1 = "20" y1 = "21" x2 = "20" y2 = "16" />< line
        xmlns = "http://www.w3.org/2000/svg" x1 = "20" y1 = "12" x2 = "20" y2 = "3" /><
        line xmlns = "http://www.w3.org/2000/svg" x1 = "1" y1 = "14" x2 = "7" y2 = "14"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "9" y1 = "8" x2 = "15" y2 =
        "8" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "17" y1 = "16" x2 = "23"
        y2 = "16" /> < title > { title } < / title > < / svg >
    }
}
