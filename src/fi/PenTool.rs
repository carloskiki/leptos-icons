#[cfg(feature = "FiPenTool")]
use leptos::*;
#[cfg(feature = "FiPenTool")]
///This icon requires the feature `FiPenTool` to be enabled.
#[component]
pub fn PenTool(
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
        "M12 19l7-7 3 3-7 7-3-3z" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M18 13l-1.5-7.5L2 2l3.5 14.5L13 18l5-5z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M2 2l7.586 7.586" />< circle xmlns =
        "http://www.w3.org/2000/svg" cx = "11" cy = "11" r = "2" /> < title > { title } <
        / title > < / svg >
    }
}
