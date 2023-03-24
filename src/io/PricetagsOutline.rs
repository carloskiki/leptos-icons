#[cfg(feature = "IoPricetagsOutline")]
use leptos::*;
#[cfg(feature = "IoPricetagsOutline")]
///This icon requires the feature `IoPricetagsOutline` to be enabled.
#[component]
pub fn PricetagsOutline(
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
        "M403.29,32H280.36a14.46,14.46,0,0,0-10.2,4.2L24.4,281.9a28.85,28.85,0,0,0,0,40.7l117,117a28.86,28.86,0,0,0,40.71,0L427.8,194a14.46,14.46,0,0,0,4.2-10.2V60.8A28.66,28.66,0,0,0,403.29,32Z"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M352,144a32,32,0,1,1,32-32A32,32,0,0,1,352,144Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M230,480,492,218a13.81,13.81,0,0,0,4-10V80"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        /> < title > { title } < / title > < / svg >
    }
}
