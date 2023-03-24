#[cfg(feature = "FiShuffle")]
use leptos::*;
#[cfg(feature = "FiShuffle")]
///This icon requires the feature `FiShuffle` to be enabled.
#[component]
pub fn Shuffle(
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
        "http://www.w3.org/2000/svg" > < polyline xmlns = "http://www.w3.org/2000/svg"
        points = "16 3 21 3 21 8" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "4"
        y1 = "20" x2 = "21" y2 = "3" />< polyline xmlns = "http://www.w3.org/2000/svg"
        points = "21 16 21 21 16 21" />< line xmlns = "http://www.w3.org/2000/svg" x1 =
        "15" y1 = "15" x2 = "21" y2 = "21" />< line xmlns = "http://www.w3.org/2000/svg"
        x1 = "4" y1 = "4" x2 = "9" y2 = "9" /> < title > { title } < / title > < / svg >
    }
}
