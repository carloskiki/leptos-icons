#[cfg(feature = "IoBicycleOutline")]
use leptos::*;
#[cfg(feature = "IoBicycleOutline")]
///This icon requires the feature `IoBicycleOutline` to be enabled.
#[component]
pub fn BicycleOutline(
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
        "M388,288a76,76,0,1,0,76,76,76.24,76.24,0,0,0-76-76Z" style =
        "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M124,288a76,76,0,1,0,76,76,76.24,76.24,0,0,0-76-76Z" style =
        "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" />< polyline xmlns
        = "http://www.w3.org/2000/svg" points =
        "256 360 256 274 192 232 272 144 312 216 368 216" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M320,136a31.89,31.89,0,0,0,32-32.1A31.55,31.55,0,0,0,320.2,72a32,32,0,1,0-.2,64Z"
        /> < title > { title } < / title > < / svg >
    }
}
