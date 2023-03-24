#[cfg(feature = "FiBellOff")]
use leptos::*;
#[cfg(feature = "FiBellOff")]
///This icon requires the feature `FiBellOff` to be enabled.
#[component]
pub fn BellOff(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M13.73 21a2 2 0 0 1-3.46 0" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M18.63 13A17.89 17.89 0 0 1 18 8" />< path xmlns = "http://www.w3.org/2000/svg"
        d = "M6.26 6.26A5.86 5.86 0 0 0 6 8c0 7-3 9-3 9h14" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 8a6 6 0 0 0-9.33-5" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "1" y1 = "1" x2 = "23" y2 = "23" /> < title > {
        title } < / title > < / svg >
    }
}
