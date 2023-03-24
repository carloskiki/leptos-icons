#[cfg(feature = "IoStopwatchOutline")]
use leptos::*;
#[cfg(feature = "IoStopwatchOutline")]
///This icon requires the feature `IoStopwatchOutline` to be enabled.
#[component]
pub fn StopwatchOutline(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < line xmlns =
        "http://www.w3.org/2000/svg" x1 = "256" y1 = "232" x2 = "256" y2 = "152" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "256" y1 = "88" x2 = "256" y2
        = "72" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:48px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "132" y1 = "132" x2 = "120" y2
        = "120" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:48px"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "256" cy = "272" r = "32"
        style = "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M256,96A176,176,0,1,0,432,272,176,176,0,0,0,256,96Z" style =
        "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" /> < title > {
        title } < / title > < / svg >
    }
}
