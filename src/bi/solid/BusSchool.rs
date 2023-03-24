#[cfg(feature = "BiSolidBusSchool")]
use leptos::*;
#[cfg(feature = "BiSolidBusSchool")]
///This icon requires the feature `BiSolidBusSchool` to be enabled.
#[component]
pub fn BusSchool(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M21 11.597V11h1V8h-1V6c0-2.206-1.794-4-4-4H7C4.794 2 3 3.794 3 6v2H2v3h1v.597a3.97 3.97 0 0 0-.999 2.648l.004 3.758c.001.733.404 1.369.995 1.716V21a1 1 0 0 0 1 1h1a1 1 0 0 0 1-1v-1h12v1a1 1 0 0 0 1 1h1a1 1 0 0 0 1-1v-1.274a2.02 2.02 0 0 0 .421-.313c.377-.378.585-.881.584-1.415l-.004-3.759A3.965 3.965 0 0 0 21 11.597zM8 4h8v2H8V4zM6.5 17a1.5 1.5 0 1 1 .001-3.001A1.5 1.5 0 0 1 6.5 17zm4.5-5H5V8h6v4zm6.5 5a1.5 1.5 0 1 1 .001-3.001A1.5 1.5 0 0 1 17.5 17zm1.5-5h-6V8h6v4z"
        /> < title > { title } < / title > < / svg >
    }
}
