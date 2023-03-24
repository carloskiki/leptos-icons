#[cfg(feature = "IoBicycle")]
use leptos::*;
#[cfg(feature = "IoBicycle")]
///This icon requires the feature `IoBicycle` to be enabled.
#[component]
pub fn Bicycle(
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
        "M388,448a92,92,0,1,1,92-92A92.1,92.1,0,0,1,388,448Zm0-152a60,60,0,1,0,60,60A60.07,60.07,0,0,0,388,296Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M124,448a92,92,0,1,1,92-92A92.1,92.1,0,0,1,124,448Zm0-152a60,60,0,1,0,60,60A60.07,60.07,0,0,0,124,296Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M320,128a31.89,31.89,0,0,0,32-32.1A31.55,31.55,0,0,0,320.2,64a32,32,0,1,0-.2,64Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M367.55,192H323.79a4,4,0,0,1-3.51-2.08l-31.74-58.17h0A31,31,0,0,0,239.16,124h0L169.3,194.4a32.56,32.56,0,0,0-9.3,22.4c0,17.4,12.6,23.6,18.5,27.1C207,260.32,227.07,272.33,238.08,279a4,4,0,0,1,1.92,3.41v69.12c0,8.61,6.62,16,15.23,16.43A16,16,0,0,0,272,352V266a16,16,0,0,0-6.66-13l-37-26.61a4,4,0,0,1-.58-6l42-44.79a4,4,0,0,1,6.42.79L298,215.77A16,16,0,0,0,312,224h56a16,16,0,0,0,16-16.77C383.58,198.62,376.16,192,367.55,192Z"
        /> < title > { title } < / title > < / svg >
    }
}
