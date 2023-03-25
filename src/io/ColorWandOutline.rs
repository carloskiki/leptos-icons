#[cfg(feature = "IoColorWandOutline")]
use leptos::*;
#[cfg(feature = "IoColorWandOutline")]
///This icon requires the feature `IoColorWandOutline` to be enabled.
#[component]
pub fn ColorWandOutline(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < rect xmlns =
        "http://www.w3.org/2000/svg" fill = "none" stroke = "#000" stroke - miterlimit =
        "10" stroke - width = "32" x = "280.48" y = "122.9" width = "63.03" height =
        "378.2" rx = "31.52" transform = "translate(-129.23 312) rotate(-45)" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M178.38,178.38h0a31.64,31.64,0,0,0,0,44.75L223.25,268,268,223.25l-44.87-44.87A31.64,31.64,0,0,0,178.38,178.38Z"
        />< line xmlns = "http://www.w3.org/2000/svg" stroke = "#000" stroke - miterlimit
        = "10" stroke - width = "32" stroke - linecap = "round" x1 = "48" y1 = "192" x2 =
        "96" y2 = "192" />< line xmlns = "http://www.w3.org/2000/svg" stroke = "#000"
        stroke - miterlimit = "10" stroke - width = "32" stroke - linecap = "round" x1 =
        "90.18" y1 = "90.18" x2 = "124.12" y2 = "124.12" />< line xmlns =
        "http://www.w3.org/2000/svg" stroke = "#000" stroke - miterlimit = "10" stroke -
        width = "32" stroke - linecap = "round" x1 = "192" y1 = "48" x2 = "192" y2 = "96"
        />< line xmlns = "http://www.w3.org/2000/svg" stroke = "#000" stroke - miterlimit
        = "10" stroke - width = "32" stroke - linecap = "round" x1 = "293.82" y1 =
        "90.18" x2 = "259.88" y2 = "124.12" />< line xmlns = "http://www.w3.org/2000/svg"
        stroke = "#000" stroke - miterlimit = "10" stroke - width = "32" stroke - linecap
        = "round" x1 = "124.12" y1 = "259.88" x2 = "90.18" y2 = "293.82" /> < title > {
        title } < / title > < / svg >
    }
}
