#[cfg(feature = "HiMdSolidMicrophone")]
use leptos::*;
#[cfg(feature = "HiMdSolidMicrophone")]
///This icon requires the feature `HiMdSolidMicrophone` to be enabled.
#[component]
pub fn Microphone(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M7 4C7 2.34315 8.34315 1 10 1C11.6569 1 13 2.34315 13 4V10C13 11.6569 11.6569 13 10 13C8.34315 13 7 11.6569 7 10V4Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5.5 9.64282C5.5 9.22861 5.16421 8.89282 4.75 8.89282C4.33579 8.89282 4 9.22861 4 9.64282V9.99997C4 13.0597 6.29027 15.5845 9.25 15.9535V17.5H7.75C7.33579 17.5 7 17.8358 7 18.25C7 18.6642 7.33579 19 7.75 19H12.25C12.6642 19 13 18.6642 13 18.25C13 17.8358 12.6642 17.5 12.25 17.5H10.75V15.9535C13.7097 15.5845 16 13.0597 16 9.99997V9.64282C16 9.22861 15.6642 8.89282 15.25 8.89282C14.8358 8.89282 14.5 9.22861 14.5 9.64282V9.99997C14.5 12.4852 12.4853 14.5 10 14.5C7.51472 14.5 5.5 12.4852 5.5 9.99997V9.64282Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
