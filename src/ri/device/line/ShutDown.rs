#[cfg(feature = "RiDeviceLineShutDown")]
use leptos::*;
#[cfg(feature = "RiDeviceLineShutDown")]
///This icon requires the feature `RiDeviceLineShutDown` to be enabled.
#[component]
pub fn ShutDown(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M6.265 3.807l1.147 1.639a8 8 0 1 0 9.176 0l1.147-1.639A9.988 9.988 0 0 1 22 12c0 5.523-4.477 10-10 10S2 17.523 2 12a9.988 9.988 0 0 1 4.265-8.193zM11 12V2h2v10h-2z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
