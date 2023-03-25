#[cfg(feature = "IoHandRightOutline")]
use leptos::*;
#[cfg(feature = "IoHandRightOutline")]
///This icon requires the feature `IoHandRightOutline` to be enabled.
#[component]
pub fn HandRightOutline(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M432,320V144a32,32,0,0,0-32-32h0a32,32,0,0,0-32,32V256" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M368,256V80a32,32,0,0,0-32-32h0a32,32,0,0,0-32,32V240" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M240,241V96a32,32,0,0,0-32-32h0a32,32,0,0,0-32,32V320" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M304,240V48a32,32,0,0,0-32-32h0a32,32,0,0,0-32,32V240" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M432,320c0,117.4-64,176-152,176s-123.71-39.6-144-88L83.33,264c-6.66-18.05-3.64-34.79,11.87-43.6h0c15.52-8.82,35.91-4.28,44.31,11.68L176,320"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        /> < title > { title } < / title > < / svg >
    }
}
