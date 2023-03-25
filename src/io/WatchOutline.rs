#[cfg(feature = "IoWatchOutline")]
use leptos::*;
#[cfg(feature = "IoWatchOutline")]
///This icon requires the feature `IoWatchOutline` to be enabled.
#[component]
pub fn WatchOutline(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < rect xmlns = "http://www.w3.org/2000/svg" x =
        "112" y = "112" width = "288" height = "288" rx = "64" ry = "64" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M176,112V40a8,8,0,0,1,8-8H328a8,8,0,0,1,8,8v72"
        style = "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M336,400v72a8,8,0,0,1-8,8H184a8,8,0,0,1-8-8V400" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /> < title > {
        title } < / title > < / svg >
    }
}
