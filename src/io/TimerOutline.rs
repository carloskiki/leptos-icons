#[cfg(feature = "IoTimerOutline")]
use leptos::*;
#[cfg(feature = "IoTimerOutline")]
///This icon requires the feature `IoTimerOutline` to be enabled.
#[component]
pub fn TimerOutline(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M112.91,128A191.85,191.85,0,0,0,64,254c-1.18,106.35,85.65,193.8,192,194,106.2.2,192-85.83,192-192,0-104.54-83.55-189.61-187.5-192A4.36,4.36,0,0,0,256,68.37V152"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M233.38,278.63l-79-113a8.13,8.13,0,0,1,11.32-11.32l113,79a32.5,32.5,0,0,1-37.25,53.26A33.21,33.21,0,0,1,233.38,278.63Z"
        /> < title > { title } < / title > < / svg >
    }
}
