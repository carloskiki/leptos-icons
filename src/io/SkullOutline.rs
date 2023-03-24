#[cfg(feature = "IoSkullOutline")]
use leptos::*;
#[cfg(feature = "IoSkullOutline")]
///This icon requires the feature `IoSkullOutline` to be enabled.
#[component]
pub fn SkullOutline(
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
        "M448,225.64v99a64,64,0,0,1-40.23,59.42l-23.68,9.47A32,32,0,0,0,364.6,417l-10,50.14A16,16,0,0,1,338.88,480H173.12a16,16,0,0,1-15.69-12.86L147.4,417a32,32,0,0,0-19.49-23.44l-23.68-9.47A64,64,0,0,1,64,324.67V224C64,118.08,149.77,32.19,255.65,32S448,119.85,448,225.64Z"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "168" cy = "280" r = "40"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "344" cy = "280" r = "40"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"
        />< polygon xmlns = "http://www.w3.org/2000/svg" points =
        "256 336 240 384 272 384 256 336" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "256" y1 = "448" x2 = "256" y2
        = "480" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "208" y1 = "448" x2 = "208" y2
        = "480" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "304" y1 = "448" x2 = "304" y2
        = "480" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        /> < title > { title } < / title > < / svg >
    }
}
