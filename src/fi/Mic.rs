#[cfg(feature = "FiMic")]
use leptos::*;
#[cfg(feature = "FiMic")]
///This icon requires the feature `FiMic` to be enabled.
#[component]
pub fn Mic(
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
        "M12 1a3 3 0 0 0-3 3v8a3 3 0 0 0 6 0V4a3 3 0 0 0-3-3z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19 10v2a7 7 0 0 1-14 0v-2" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "12" y1 = "19" x2 = "12" y2 = "23" />< line
        xmlns = "http://www.w3.org/2000/svg" x1 = "8" y1 = "23" x2 = "16" y2 = "23" /> <
        title > { title } < / title > < / svg >
    }
}
