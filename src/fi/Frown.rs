#[cfg(feature = "FiFrown")]
use leptos::*;
#[cfg(feature = "FiFrown")]
///This icon requires the feature `FiFrown` to be enabled.
#[component]
pub fn Frown(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = "24" height = "24"
        fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap =
        "round" stroke - linejoin = "round" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < circle xmlns = "http://www.w3.org/2000/svg" cx =
        "12" cy = "12" r = "10" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M16 16s-1.5-2-4-2-4 2-4 2" />< line xmlns = "http://www.w3.org/2000/svg" x1 =
        "9" y1 = "9" x2 = "9.01" y2 = "9" />< line xmlns = "http://www.w3.org/2000/svg"
        x1 = "15" y1 = "9" x2 = "15.01" y2 = "9" /> < title > { title } < / title > < /
        svg >
    }
}
