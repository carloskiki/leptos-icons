#[cfg(feature = "IoFootstepsOutline")]
use leptos::*;
#[cfg(feature = "IoFootstepsOutline")]
///This icon requires the feature `IoFootstepsOutline` to be enabled.
#[component]
pub fn FootstepsOutline(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style id = "icons" viewBox = "0 0 512 512" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M200,246.84c8.81,58.62-7.33,90.67-52.91,97.41-50.65,7.49-71.52-26.44-80.33-85.06-11.85-78.88,16-127.94,55.71-131.1C158.61,125.22,191.18,188.23,200,246.84Z"
        fill = "none" stroke = "#000" stroke - miterlimit = "10" stroke - width = "32"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M223.65,409.53c3.13,33.28-14.86,64.34-42,69.66-27.4,5.36-58.71-16.37-65.09-49.19s17.75-34.56,47.32-40.21S219.87,369.39,223.65,409.53Z"
        fill = "none" stroke = "#000" stroke - miterlimit = "10" stroke - width = "32"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M312,150.83c-8.81,58.62,7.33,90.67,52.9,97.41,50.66,7.49,71.52-26.44,80.33-85.06,11.86-78.89-16-128.22-55.7-131.1C353.13,29.44,320.82,92.21,312,150.83Z"
        fill = "none" stroke = "#000" stroke - miterlimit = "10" stroke - width = "32"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M288.35,313.53c-3.13,33.27,14.86,64.34,42,69.66,27.4,5.36,58.71-16.37,65.09-49.19s-17.75-34.56-47.32-40.22S292.13,273.38,288.35,313.53Z"
        fill = "none" stroke = "#000" stroke - miterlimit = "10" stroke - width = "32" />
        < title > { title } < / title > < / svg >
    }
}
