#[cfg(feature = "IoVideocam")]
use leptos::*;
#[cfg(feature = "IoVideocam")]
///This icon requires the feature `IoVideocam` to be enabled.
#[component]
pub fn Videocam(
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
        "M464,384.39a32,32,0,0,1-13-2.77,15.77,15.77,0,0,1-2.71-1.54l-82.71-58.22h0A32,32,0,0,1,352,295.7V216.3a32,32,0,0,1,13.58-26.16l82.71-58.22a15.77,15.77,0,0,1,2.71-1.54,32,32,0,0,1,45,29.24V352.38a32,32,0,0,1-32,32Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M268,400H84a68.07,68.07,0,0,1-68-68V180a68.07,68.07,0,0,1,68-68H268.48A67.6,67.6,0,0,1,336,179.52V332A68.07,68.07,0,0,1,268,400Z"
        /> < title > { title } < / title > < / svg >
    }
}
