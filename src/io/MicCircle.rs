#[cfg(feature = "IoMicCircle")]
use leptos::*;
#[cfg(feature = "IoMicCircle")]
///This icon requires the feature `IoMicCircle` to be enabled.
#[component]
pub fn MicCircle(
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
        "M256,48C141.31,48,48,141.31,48,256s93.31,208,208,208,208-93.31,208-208S370.69,48,256,48ZM208,176a48.14,48.14,0,0,1,48-48h0a48.14,48.14,0,0,1,48,48v64a48.14,48.14,0,0,1-48,48h0a48.14,48.14,0,0,1-48-48Zm144,72.22c0,23.36-10.94,45.61-30.79,62.66A103.71,103.71,0,0,1,272,334.26V352h16a16,16,0,0,1,0,32H224a16,16,0,0,1,0-32h16V334.26a103.71,103.71,0,0,1-49.21-23.38C170.94,293.83,160,271.58,160,248.22V224.3a16,16,0,0,1,32,0v23.92c0,25.66,28,55.48,64,55.48,29.6,0,64-24.23,64-55.48V224.3a16,16,0,1,1,32,0Z"
        /> < title > { title } < / title > < / svg >
    }
}
