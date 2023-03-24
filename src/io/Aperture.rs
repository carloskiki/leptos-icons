#[cfg(feature = "IoAperture")]
use leptos::*;
#[cfg(feature = "IoAperture")]
///This icon requires the feature `IoAperture` to be enabled.
#[component]
pub fn Aperture(
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
        "M250.54,129.17l-67.8-67.8A209.65,209.65,0,0,0,86.32,136h161.4A4,4,0,0,0,250.54,129.17Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M167.72,168H67.63A207.34,207.34,0,0,0,51.48,293.9L170.54,174.83A4,4,0,0,0,167.72,168Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M344,167.72V67.56A207.82,207.82,0,0,0,218.11,51.48L337.17,170.54A4,4,0,0,0,344,167.72Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M460.52,218.1,341.46,337.17a4,4,0,0,0,2.82,6.83H444.37a207.34,207.34,0,0,0,16.15-125.9Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M382.83,250.54l67.83-67.82A209.08,209.08,0,0,0,376,86.2V247.72A4,4,0,0,0,382.83,250.54Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M221.68,341.77a8,8,0,0,0,5.54,2.23h59.66a8,8,0,0,0,5.7-2.39l49.18-50a8,8,0,0,0,2.3-5.62L344,225.18a8,8,0,0,0-2.38-5.69l-50-49.25a8,8,0,0,0-5.63-2.3l-60.84.06a8,8,0,0,0-5.69,2.38l-49.25,50a8,8,0,0,0-2.3,5.63l.06,60.78a8,8,0,0,0,2.45,5.76Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M261.46,382.83l67.8,67.8A209.65,209.65,0,0,0,425.68,376H264.28A4,4,0,0,0,261.46,382.83Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M168,344.28V444.44a207.82,207.82,0,0,0,125.89,16.08L174.83,341.46A4,4,0,0,0,168,344.28Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M129.17,261.46,61.34,329.29A209.1,209.1,0,0,0,136,425.8V264.28A4,4,0,0,0,129.17,261.46Z"
        /> < title > { title } < / title > < / svg >
    }
}
