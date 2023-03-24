#[cfg(feature = "IoCodeWorking")]
use leptos::*;
#[cfg(feature = "IoCodeWorking")]
///This icon requires the feature `IoCodeWorking` to be enabled.
#[component]
pub fn CodeWorking(
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
        "0 0 512 512" width = { size.clone() } height = { size } > < circle xmlns =
        "http://www.w3.org/2000/svg" cx = "256" cy = "256" r = "26" style =
        "stroke:#000;stroke-miterlimit:10;stroke-width:10px" />< circle xmlns =
        "http://www.w3.org/2000/svg" cx = "346" cy = "256" r = "26" style =
        "stroke:#000;stroke-miterlimit:10;stroke-width:10px" />< circle xmlns =
        "http://www.w3.org/2000/svg" cx = "166" cy = "256" r = "26" style =
        "stroke:#000;stroke-miterlimit:10;stroke-width:10px" />< polyline xmlns =
        "http://www.w3.org/2000/svg" points = "160 368 32 256 160 144" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:42px"
        />< polyline xmlns = "http://www.w3.org/2000/svg" points =
        "352 368 480 256 352 144" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:42px"
        /> < title > { title } < / title > < / svg >
    }
}
