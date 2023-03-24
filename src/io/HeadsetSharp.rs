#[cfg(feature = "IoHeadsetSharp")]
use leptos::*;
#[cfg(feature = "IoHeadsetSharp")]
///This icon requires the feature `IoHeadsetSharp` to be enabled.
#[component]
pub fn HeadsetSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M411.16,97.45C368.43,55.85,311.88,32,256,32S143.57,55.85,100.84,97.45C56.45,140.67,32,196,32,256S58.84,374.49,77.42,408.25,121,480,144,480c32,0,96-32,96-32L128,240,73.58,271.73a176.07,176.07,0,0,1-1-18.84c0-48.57,19.32-94.1,56.15-130C164.24,88.34,210,70,256,70s91.73,18.34,127.27,52.93c36.83,35.86,56.14,81.39,56.14,130a175.56,175.56,0,0,1-1,18.82L384,240,272,448s64,32,96,32c23,0,48-38,66.58-71.75S480,316,480,256,455.55,140.67,411.16,97.45Z"
        /> < title > { title } < / title > < / svg >
    }
}
