#[cfg(feature = "IoInformationCircleOutline")]
use leptos::*;
#[cfg(feature = "IoInformationCircleOutline")]
///This icon requires the feature `IoInformationCircleOutline` to be enabled.
#[component]
pub fn InformationCircleOutline(
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
        "M248,64C146.39,64,64,146.39,64,248s82.39,184,184,184,184-82.39,184-184S349.61,64,248,64Z"
        style = "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" /><
        polyline xmlns = "http://www.w3.org/2000/svg" points = "220 220 252 220 252 336"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "208" y1 = "340" x2 = "296" y2
        = "340" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M248,130a26,26,0,1,0,26,26A26,26,0,0,0,248,130Z" /> < title > { title } < /
        title > < / svg >
    }
}
