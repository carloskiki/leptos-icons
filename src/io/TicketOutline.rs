#[cfg(feature = "IoTicketOutline")]
use leptos::*;
#[cfg(feature = "IoTicketOutline")]
///This icon requires the feature `IoTicketOutline` to be enabled.
#[component]
pub fn TicketOutline(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" fill = "none" stroke = "#000" stroke - miterlimit =
        "10" stroke - width = "32" d =
        "M366.05,146a46.7,46.7,0,0,1-2.42-63.42,3.87,3.87,0,0,0-.22-5.26L319.28,33.14a3.89,3.89,0,0,0-5.5,0l-70.34,70.34a23.62,23.62,0,0,0-5.71,9.24h0a23.66,23.66,0,0,1-14.95,15h0a23.7,23.7,0,0,0-9.25,5.71L33.14,313.78a3.89,3.89,0,0,0,0,5.5l44.13,44.13a3.87,3.87,0,0,0,5.26.22,46.69,46.69,0,0,1,65.84,65.84,3.87,3.87,0,0,0,.22,5.26l44.13,44.13a3.89,3.89,0,0,0,5.5,0l180.4-180.39a23.7,23.7,0,0,0,5.71-9.25h0a23.66,23.66,0,0,1,14.95-15h0a23.62,23.62,0,0,0,9.24-5.71l70.34-70.34a3.89,3.89,0,0,0,0-5.5l-44.13-44.13a3.87,3.87,0,0,0-5.26-.22A46.7,46.7,0,0,1,366.05,146Z"
        />< line xmlns = "http://www.w3.org/2000/svg" fill = "none" stroke = "#000"
        stroke - miterlimit = "10" stroke - width = "32" stroke - linecap = "round" x1 =
        "250.5" y1 = "140.44" x2 = "233.99" y2 = "123.93" />< line xmlns =
        "http://www.w3.org/2000/svg" fill = "none" stroke = "#000" stroke - miterlimit =
        "10" stroke - width = "32" stroke - linecap = "round" x1 = "294.52" y1 = "184.46"
        x2 = "283.51" y2 = "173.46" />< line xmlns = "http://www.w3.org/2000/svg" fill =
        "none" stroke = "#000" stroke - miterlimit = "10" stroke - width = "32" stroke -
        linecap = "round" x1 = "338.54" y1 = "228.49" x2 = "327.54" y2 = "217.48" /><
        line xmlns = "http://www.w3.org/2000/svg" fill = "none" stroke = "#000" stroke -
        miterlimit = "10" stroke - width = "32" stroke - linecap = "round" x1 = "388.07"
        y1 = "278.01" x2 = "371.56" y2 = "261.5" /> < title > { title } < / title > < /
        svg >
    }
}
