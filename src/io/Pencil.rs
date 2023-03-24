#[cfg(feature = "IoPencil")]
use leptos::*;
#[cfg(feature = "IoPencil")]
///This icon requires the feature `IoPencil` to be enabled.
#[component]
pub fn Pencil(
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
        "0 0 512 512" width = { size.clone() } height = { size } > < polygon xmlns =
        "http://www.w3.org/2000/svg" points =
        "358.62 129.28 86.49 402.08 70 442 109.92 425.51 382.72 153.38 358.62 129.28"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:44px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M413.07,74.84,401.28,86.62l24.1,24.1,11.79-11.79a16.51,16.51,0,0,0,0-23.34l-.75-.75A16.51,16.51,0,0,0,413.07,74.84Z"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:44px"
        /> < title > { title } < / title > < / svg >
    }
}
