#[cfg(feature = "FiMove")]
use leptos::*;
#[cfg(feature = "FiMove")]
///This icon requires the feature `FiMove` to be enabled.
#[component]
pub fn Move(
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
        points = "5 9 2 12 5 15" />< polyline xmlns = "http://www.w3.org/2000/svg" points
        = "9 5 12 2 15 5" />< polyline xmlns = "http://www.w3.org/2000/svg" points =
        "15 19 12 22 9 19" />< polyline xmlns = "http://www.w3.org/2000/svg" points =
        "19 9 22 12 19 15" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "2" y1 =
        "12" x2 = "22" y2 = "12" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "12"
        y1 = "2" x2 = "12" y2 = "22" /> < title > { title } < / title > < / svg >
    }
}
