#[cfg(feature = "FaSolidPlaneCircleExclamation")]
use leptos::*;
#[cfg(feature = "FaSolidPlaneCircleExclamation")]
///This icon requires the feature `FaSolidPlaneCircleExclamation` to be enabled.
#[component]
pub fn PlaneCircleExclamation(
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
        stroke_witdh = "0" style = style viewBox = "0 0 640 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M256 0c-35 0-64 59.5-64 93.7v84.6L8.1 283.4c-5 2.8-8.1 8.2-8.1 13.9v65.5c0 10.6 10.2 18.3 20.4 15.4l171.6-49 0 70.9-57.6 43.2c-4 3-6.4 7.8-6.4 12.8v42c0 7.8 6.3 14 14 14c1.3 0 2.6-.2 3.9-.5L256 480l110.1 31.5c1.3 .4 2.6 .5 3.9 .5c6 0 11.1-3.7 13.1-9C344.5 470.7 320 422.2 320 368c0-60.6 30.6-114 77.1-145.6L320 178.3V93.7C320 59.5 292 0 256 0zM496 512a144 144 0 1 0 0-288 144 144 0 1 0 0 288zm0-96a24 24 0 1 1 0 48 24 24 0 1 1 0-48zm0-144c8.8 0 16 7.2 16 16v80c0 8.8-7.2 16-16 16s-16-7.2-16-16V288c0-8.8 7.2-16 16-16z"
        /> < title > { title } < / title > < / svg >
    }
}
