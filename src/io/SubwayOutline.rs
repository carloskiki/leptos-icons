#[cfg(feature = "IoSubwayOutline")]
use leptos::*;
#[cfg(feature = "IoSubwayOutline")]
///This icon requires the feature `IoSubwayOutline` to be enabled.
#[component]
pub fn SubwayOutline(
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
        "112" y = "32" width = "288" height = "352" rx = "48" ry = "48" style =
        "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "208" y1 = "80" x2 = "304" y2 = "80" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< rect xmlns = "http://www.w3.org/2000/svg" x = "112" y = "128" width = "288"
        height = "96" rx = "32" ry = "32" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "176" cy = "320" r = "16"
        style = "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /><
        circle xmlns = "http://www.w3.org/2000/svg" cx = "336" cy = "320" r = "16" style
        = "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" />< line xmlns
        = "http://www.w3.org/2000/svg" x1 = "144" y1 = "464" x2 = "368" y2 = "464" style
        =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "336" y1 = "432" x2 = "384" y2
        = "480" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "176" y1 = "432" x2 = "128" y2
        = "480" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        /> < title > { title } < / title > < / svg >
    }
}
