#[cfg(feature = "FiLoader")]
use leptos::*;
#[cfg(feature = "FiLoader")]
///This icon requires the feature `FiLoader` to be enabled.
#[component]
pub fn Loader(
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
        "12" y1 = "2" x2 = "12" y2 = "6" />< line xmlns = "http://www.w3.org/2000/svg" x1
        = "12" y1 = "18" x2 = "12" y2 = "22" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "4.93" y1 = "4.93" x2 = "7.76" y2 = "7.76" /><
        line xmlns = "http://www.w3.org/2000/svg" x1 = "16.24" y1 = "16.24" x2 = "19.07"
        y2 = "19.07" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "2" y1 = "12" x2
        = "6" y2 = "12" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "18" y1 = "12"
        x2 = "22" y2 = "12" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "4.93" y1
        = "19.07" x2 = "7.76" y2 = "16.24" />< line xmlns = "http://www.w3.org/2000/svg"
        x1 = "16.24" y1 = "7.76" x2 = "19.07" y2 = "4.93" /> < title > { title } < /
        title > < / svg >
    }
}
